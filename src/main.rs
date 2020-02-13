mod cli;
mod commands;
mod util;

use clap::Values;

fn main() {
    let matches = cli::get_app().get_matches();

    match matches.subcommand() {
        ("new", Some(new_matches)) => match new_matches.subcommand() {
            ("api", Some(api_matches)) => {
                commands::new::on_new_api(String::from(api_matches.value_of("api_name").unwrap()));
            }
            ("env", Some(env_matches)) => {
                commands::new::on_new_env(
                    String::from(env_matches.value_of("api_name").unwrap()),
                    String::from(env_matches.value_of("env_name").unwrap()),
                    String::from(env_matches.value_of("api_host").unwrap()),
                );
            }
            ("action", Some(action_matches)) => {
                commands::new::on_new_action(
                    String::from(action_matches.value_of("api_name").unwrap()),
                    String::from(action_matches.value_of("env_name").unwrap()),
                    String::from(action_matches.value_of("action_name").unwrap()),
                    String::from(action_matches.value_of("path").unwrap()),
                    String::from(action_matches.value_of("method").unwrap()),
                    String::from(action_matches.value_of("payload").unwrap_or("")),
                );
            }
            _ => unreachable!(),
        },
        ("run", Some(run_matches)) => commands::run::on_run(
            String::from(run_matches.value_of("api_name").unwrap()),
            String::from(run_matches.value_of("env_name").unwrap()),
            String::from(run_matches.value_of("action_name").unwrap()),
            &run_matches
                .values_of("action_arg_list")
                .unwrap_or(Values::default())
                .collect(),
        ),
        ("init", Some(_)) => commands::init::on_init(),
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
