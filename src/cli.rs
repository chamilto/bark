use clap::{App, AppSettings, Arg};

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    return App::new("bark")
            .about("Action-Based HTTP API Client")
            .version("1.0")
            .author("chamilto")
            .subcommand(
                App::new("new")
                    .about("Creates an API or an action.")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(
                        App::new("api")
                            .about("generates a new api namespace to which you can attach actions.")
                            .arg(
                                Arg::with_name("api_name")
                                    .required(true)
                                    .help("The name you wish to give the API namespace."),
                            ),
                    )
                    .subcommand(
                        App::new("env")
                            .about("Creates an environment for an API, e.g. QA.")
                            .arg(
                                Arg::with_name("api_name")
                                    .required(true)
                                    .help("The name of the API for which you want to create an environment."),
                            )
                            .arg(
                                Arg::with_name("env_name")
                                    .required(true)
                                    .help("The name of the environment."),
                            )
                            .arg(
                                Arg::with_name("api_host")
                                    .required(true)
                                    .help("The API host address for the environment."),
                            ),
                    )
                    .subcommand(
                        App::new("action")
                            .about("")
                            .arg(Arg::with_name("api_name").required(true).help(
                                "The name of the API for which you wish to create an action.",
                            ))
                            .arg(Arg::with_name("env_name").required(true).help(
                                "The name of the env for which you wish to create an action.",
                            ))
                            .arg(
                                Arg::with_name("action_name")
                                    .required(true)
                                    .long("name")
                                    .takes_value(true)
                                    .value_name("action_name")
                                    .help(""),
                            )
                            .arg(
                                Arg::with_name("path")
                                    .required(true)
                                    .long("path")
                                    .takes_value(true)
                                    .value_name("path")
                                    .help(""),
                            )
                            .arg(
                                Arg::with_name("method")
                                    .required(true)
                                    .long("method")
                                    .takes_value(true)
                                    .value_name("method")
                                    .help(""),
                            )
                            .arg(
                                Arg::with_name("payload")
                                    .required(true)
                                    .long("payload")
                                    .takes_value(true)
                                    .value_name("payload")
                                    .help(""),
                            ),
                    ),
            )
            .subcommand(
                App::new("run")
                    .about("Runs an action.")
                    .arg(
                        Arg::with_name("api_name")
                            .required(true)
                            .help("The name of the API to which the action belongs."),
                    )
                    .arg(
                        Arg::with_name("env_name")
                            .required(true)
                            .help("The name of the environment against which the action will be run."),
                    )
                    .arg(Arg::with_name("action_name").required(true).help(""))
                    .arg(Arg::with_name("action_arg_list").multiple(true)),
            )
            .subcommand(
                App::new("init")
                    .about("Initialize the bark action DB, create bark directories, and create the bark config file.")
            );
}
