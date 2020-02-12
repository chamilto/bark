use crate::util::bark_print;

pub fn on_run(api_name: String, env_name: String, action_name: String, args: Vec<&str>) {
    // check if api exists
    // check if action exists
    // load the path/method/host etc
    // if method is post/put - look up the payload file and load it
    // if the payload file doesn't exist - print a warning that we're posting or putting without
    // data
    // set up timer
    // make request
    // end timer
    // print response details
    let host = String::from("http://localhost:8000");
    let method = String::from("GET");
    let mut path = String::from("/tenants/{0}/users/{1}");

    // really hacky
    // has to be a better way to do this
    match args.len() {
        1 => {
            path = path.replace("{0}", args[0]);
        }
        2 => {
            path = path.replace("{0}", args[0]);
            path = path.replace("{1}", args[1]);
        }
        3 => {
            path = path.replace("{0}", args[0]);
            path = path.replace("{1}", args[1]);
            path = path.replace("{2}", args[2]);
        }
        4 => {
            path = path.replace("{0}", args[0]);
            path = path.replace("{1}", args[1]);
            path = path.replace("{2}", args[2]);
            path = path.replace("{3}", args[3]);
        }
        5 => {
            path = path.replace("{0}", args[0]);
            path = path.replace("{1}", args[1]);
            path = path.replace("{2}", args[2]);
            path = path.replace("{3}", args[3]);
            path = path.replace("{4}", args[4]);
        }
        _ => {}
    }

    bark_print(format!(
        "Executing {}-{}:{} with args {:?}",
        api_name, env_name, action_name, args
    ));
    bark_print(format!("Making {} request to {}{}", method, host, path));
}
