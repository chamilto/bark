use std::fs;
use std::path::Path;

use crate::util::{bark_print, get_bark_dir, get_db_conn};

const METHODS: [&'static str; 6] = ["GET", "PUT", "POST", "DELETE", "PATCH", "HEAD"];
const PAYLOAD_METHODS: [&'static str; 3] = ["POST", "PUT", "PATCH"];

pub fn on_new_api(api_name: String) {
    let db = get_db_conn();

    let mut cursor = db
        .prepare("SELECT 1 FROM api WHERE name = ?")
        .unwrap()
        .cursor();

    cursor
        .bind(&[sqlite::Value::String(api_name.clone())])
        .unwrap();

    if let Some(_) = cursor.next().unwrap() {
        bark_print(format!("API {} already exists.", api_name));
        return;
    }

    let bark_dir = get_bark_dir();

    if Path::new(&format!("{}/{}", &bark_dir, &api_name)).exists() {
        bark_print(format!("API {} already exists.", api_name));
        return;
    }

    match fs::create_dir_all(format!("{}/api/{}/payloads", bark_dir, api_name)) {
        Ok(()) => bark_print(format!("Created API with name {}", api_name)),
        Err(e) => bark_print(format!("Issue creating directory: {:?}", e)),
    }

    bark_print(format!("Creating API {} in bark DB", api_name));

    db.execute(format!("INSERT INTO api (name) VALUES ('{}');", api_name))
        .unwrap();

    bark_print(format!("Created API {} in bark DB", api_name));
}

pub fn on_new_env(api_name: String, env_name: String, api_host: String) {
    let db = get_db_conn();

    let mut cursor = db
        .prepare("SELECT id FROM api WHERE name = ?;")
        .unwrap()
        .cursor();

    cursor
        .bind(&[sqlite::Value::String(api_name.clone())])
        .unwrap();

    let api_row = if let Some(row) = cursor.next().unwrap() {
        row
    } else {
        bark_print(format!("No API with name {} found.", api_name));
        return;
    };

    let mut cursor = db
        .prepare("SELECT 1 FROM env JOIN api on api.id == env.api_id WHERE api.name = ? and env.name == ?;")
        .unwrap()
        .cursor();

    cursor
        .bind(&[
            sqlite::Value::String(api_name.clone()),
            sqlite::Value::String(env_name.clone()),
        ])
        .unwrap();

    if let Some(_) = cursor.next().unwrap() {
        bark_print(format!(
            "Env {} for API {} already exists.",
            env_name, api_name
        ));
        return;
    }

    db.execute(format!(
        "INSERT INTO env (api_id, name, host) VALUES ({}, '{}', '{}')",
        api_row[0].as_integer().unwrap(),
        env_name,
        api_host
    ))
    .unwrap();

    bark_print(format!(
        "Created env {} for API {} with host {}.",
        env_name, api_name, api_host
    ))
}

pub fn on_new_action(
    api_name: String,
    env_name: String,
    action_name: String,
    path: String,
    method: String,
    payload_filename: String,
) {
    let db = get_db_conn();

    let mut cursor = db
        .prepare("SELECT env.id FROM env JOIN api on env.api_id = api.id WHERE api.name = ? and env.name = ?;")
        .unwrap()
        .cursor();

    cursor
        .bind(&[
            sqlite::Value::String(api_name.clone()),
            sqlite::Value::String(env_name.clone()),
        ])
        .unwrap();

    let env_row = if let Some(row) = cursor.next().unwrap() {
        row
    } else {
        bark_print(format!(
            "No env with name {} for API {} found.",
            env_name, api_name
        ));
        return;
    };

    let mut cursor = db
        .prepare("SELECT 1 FROM action WHERE name = ? AND env_id = ?")
        .unwrap()
        .cursor();

    cursor
        .bind(&[
            sqlite::Value::String(action_name.clone()),
            sqlite::Value::Integer(env_row[0].as_integer().unwrap()),
        ])
        .unwrap();

    if let Some(_) = cursor.next().unwrap() {
        bark_print(format!(
            "Action {} for API {} env {} already exists.",
            action_name, api_name, env_name
        ));
        return;
    }

    if !METHODS.contains(&&method[..]) {
        bark_print(format!(
            "{} is not a valid method. Must be one of {:?} ",
            method, METHODS
        ));
        return;
    }

    if !PAYLOAD_METHODS.contains(&&method[..]) && payload_filename == String::from("") {
        bark_print(format!(
            "Warning: Creating action with method {} with no payload file.",
            method
        ));
    }

    let bark_dir = get_bark_dir();
    let payload_file_path = format!(
        "{}/api/{}/payloads/{}",
        bark_dir, api_name, payload_filename
    );

    if !Path::new(&payload_file_path).exists() {
        bark_print(format!("Payload file {} not found.", payload_file_path));
        return;
    }

    db.execute(format!(
        "INSERT INTO action (env_id, name, path, method, payload_filename) 
         VALUES ({}, '{}', '{}', '{}', '{}');",
        env_row[0].as_integer().unwrap(),
        action_name,
        path,
        method,
        payload_filename
    ))
    .unwrap();

    if payload_filename != String::from("") {
        bark_print(format!(
            "Created {} action {} for API {} and env {} with path {} and payload file {}.",
            method, action_name, api_name, env_name, path, payload_filename
        ));
    } else {
        bark_print(format!(
            "Created {} action {} for API {} and env {} with path {}.",
            method, action_name, api_name, env_name, path,
        ));
    }
}
