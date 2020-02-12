extern crate ini;

use ini::Ini;
use std::fs;
use std::path::Path;

use crate::util::{bark_print, get_bark_dir, get_db_conn};

fn create_init_dirs(bark_dir: &String) {
    bark_print(format!("Creating \"{}\"", bark_dir));

    if Path::new(&bark_dir).exists() {
        bark_print(format!(
            "Directory \"{}\" already exists. Skipping.",
            bark_dir
        ));
    } else {
        match fs::create_dir_all(&bark_dir) {
            Ok(()) => bark_print(format!("Created \"{}\"", bark_dir)),
            Err(e) => bark_print(format!("Issue creating \"{}\": {:?}", bark_dir, e)),
        }
    }
}

fn create_db() {
    let db = get_db_conn();

    db.execute(
        "
        CREATE TABLE IF NOT EXISTS api(
            id INTEGER PRIMARY KEY, 
            name TEXT UNIQUE
        );

        CREATE TABLE IF NOT EXISTS env(
            id INTEGER PRIMARY KEY, 
            api_id INTEGER, 
            name TEXT, 
            host TEXT,
            FOREIGN KEY(api_id) REFERENCES api(id)
        );

        CREATE TABLE IF NOT EXISTS action(
            id INTEGER PRIMARY KEY, 
            env_id INTEGER,
            name TEXT, 
            path TEXT,
            method TEXT,
            payload_filename TEXT,
            FOREIGN KEY(env_id) REFERENCES env(id)
        );
        ",
    )
    .unwrap()
}

fn create_config(bark_dir: &String) {
    let full_path = format!("{}/conf.ini", bark_dir);
    let path = Path::new(&full_path);

    bark_print(format!("Creating config file \"{}\"", full_path));

    if path.exists() {
        bark_print(format!("File \"{}\" already exists. Skipping.", full_path));
        return;
    }

    let mut conf = Ini::new();
    conf.with_section(Some("general")).set("editor", "vim");

    match conf.write_to_file(&path) {
        Ok(_) => bark_print(format!("Created \"{}\".", full_path)),
        Err(why) => panic!("couldn't write to {}: {}", full_path, why),
    }
}

pub fn on_init() {
    let bark_dir = get_bark_dir();

    create_init_dirs(&bark_dir);
    create_config(&bark_dir);
    create_db();
}
