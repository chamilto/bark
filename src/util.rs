extern crate ini;

pub fn bark_print(msg: String) {
    println!("bark: {}", msg);
}

pub fn get_db_conn() -> sqlite::Connection {
    let bark_path = get_bark_dir();
    return sqlite::open(format!("{}/bark.db", bark_path)).unwrap();
}

pub fn get_bark_dir() -> String {
    let mut home_dir = String::new();

    if let Some(home) = dirs::home_dir() {
        home_dir = String::from(home.to_str().unwrap())
    }

    return format!("{}/bark", home_dir);
}
