# bark
learning rust by building a poor man's http api cli tool.

# Top Level Commands
```
USAGE:
    bark [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    init    Initialize the bark action DB, create bark directories, and create a bark config file.
    new     Creates an api, environment or an action.
    run     Runs an action.
```
_______________________________________________________________________


## New (SUBCOMMAND)
```
Creates an api, environment or an action.

USAGE:
    bark new <SUBCOMMAND>

SUBCOMMANDS:
    action    creates a new action for an environment.
    api       generates a new api namespace inside which you can create environments.
    env       Creates an environment for an api, e.g. QA.
    help      Prints this message or the help of the given subcommand(s)
```

### new api 
```
USAGE:
    bark new api <api_name>

ARGS:
    <api_name>    The name you wish to give the api.
```

### new env
```
Creates an environment for an api, e.g. 'QA'.

USAGE:
    bark new env <api_name> <env_name> --host <api_host>

OPTIONS:
        --host <api_host>    The API host address for the environment.

ARGS:
    <api_name>    The name of the API for which you want to create an environment.
    <env_name>    The name of the environment.
```

### new action
```
bark-new-action


USAGE:
    bark new action [OPTIONS] <api_name> <env_name> --name <action_name> --method <method> --path <path>

OPTIONS:
        --name <action_name>
        --method <method>
        --path <path> (Should be formatted like '/tenants/{0}/users/{1}' if you want to interpolate values into a path.
        --payload <payload>

ARGS:
    <api_name>    The name of the api for which you wish to create an action.
    <env_name>    The name of the env for which you wish to create an action.
```
______________________________________________________________________________________


## Run (SUBCOMMAND)
```
bark-run
Runs an action.

USAGE:
    bark run <api_name> <env_name> <action_name> [action_arg_list]...

ARGS:
    <api_name>              The name of the API to which the action belongs.
    <env_name>              The name of the environment against which the action will be run.
    <action_name>           The name of the action to run.
    <action_arg_list>...    These values will be interpolated into the path in the order given ({0}, {1}, {2}, ...)
```
