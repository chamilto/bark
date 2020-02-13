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

pub fn on_run(api_name: String, env_name: String, action_name: String, args: &Vec<&str>) {
    // data
    // set up timer
    // make request
    // end timer
    // print response details
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

    bark_print(format!(
        "Executing {} (env:{}): {} with args {:?}",
        api_name, env_name, action_name, args
    ));

    if payload_filename != "" {
        bark_print(format!(
            "Making {} request to {}{} with payload {}",
            method, host, path, payload_filename
        ));
    } else {
        if ["POST", "PUT"].contains(&method) {
            bark_print(format!(
                "Warning: No payload file found. Making {} request with no request body.",
                method
            ))
        }
        bark_print(format!("Making {} request to {}{}", method, host, path,));
    }
}
