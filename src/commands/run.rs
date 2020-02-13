use std::fs::read_to_string;
use std::path::Path;

use serde_json::Value;

use crate::util::{bark_print, get_bark_dir, get_db_conn};

fn format_path(path: &mut String, args: &Vec<&str>) {
    for (idx, arg) in args.iter().enumerate() {
        *path = path.replace(&format!("{{{}}}", idx), arg);
    }
}

fn get_action_cursor<'a>(
    db: &'a sqlite::Connection,
    api_name: &String,
    env_name: &String,
    action_name: &String,
) -> sqlite::Cursor<'a> {
    let mut cursor = db
        .prepare(
            "SELECT env.host, action.method, action.path, action.payload_filename
             FROM action
             JOIN env on env.id = action.env_id
             JOIN api on env.api_id = api.id
             WHERE api.name = ? AND action.name = ? AND env.name = ? ;
        ",
        )
        .unwrap()
        .cursor();

    cursor
        .bind(&[
            sqlite::Value::String(api_name.clone()),
            sqlite::Value::String(action_name.clone()),
            sqlite::Value::String(env_name.clone()),
        ])
        .unwrap();

    cursor
}

struct RequestPayload {
    raw: Option<String>,
}

impl RequestPayload {
    fn from_file(file_path: &Path, method: &str) -> RequestPayload {
        if let Ok(contents) = read_to_string(file_path) {
            RequestPayload {
                raw: Some(contents),
            }
        } else {
            if crate::PAYLOAD_METHODS.contains(&method) {
                bark_print(format!("Warning: Unable to open {:?}", &file_path));
            }

            RequestPayload { raw: None }
        }
    }

    fn to_json(&mut self) -> Option<Value> {
        if let Some(raw) = &self.raw {
            serde_json::from_str(raw.as_str()).unwrap()
        } else {
            None
        }
    }
}

pub fn on_run(api_name: String, env_name: String, action_name: String, args: &Vec<&str>) {
    let db = get_db_conn();
    let mut cursor = get_action_cursor(&db, &api_name, &env_name, &action_name);

    let host;
    let method;
    let mut path;
    let payload_filename;

    if let Some(row) = cursor.next().unwrap() {
        host = row[0].as_string().unwrap();
        method = row[1].as_string().unwrap();
        path = row[2].as_string().unwrap().to_string();
        payload_filename = row[3].as_string().unwrap_or("");
    } else {
        bark_print(String::from("Action not found."));
        return;
    }

    format_path(&mut path, args);

    let payload_file_path_str = format!(
        "{}/api/{}/payloads/{}",
        get_bark_dir(),
        &api_name,
        &payload_filename,
    );
    let payload_file_path = Path::new(&payload_file_path_str);
    let request_payload = RequestPayload::from_file(payload_file_path, &method).to_json();

    let client = reqwest::blocking::Client::new();
    let req_method = reqwest::Method::from_bytes(method.as_bytes()).unwrap();

    let mut req = client.request(req_method, format!("{}{}", &host, &path).as_str());

    let resp;

    if let Some(payload) = &request_payload {
        req = req.json(&payload);
        bark_print(format!("{:#?}", req));
        resp = req.send().unwrap();
    } else {
        bark_print(format!("{:#?}", req));
        resp = req.send().unwrap();
    }

    bark_print(format!("Status: {}", resp.status()));
    bark_print(format!("Body: {}", resp.text().unwrap()));
}
