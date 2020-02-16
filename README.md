### getopt-long

This crate is so simple. Just see the following example:
```rust
let help_long = "help".to_string();
let config_long = "config".to_string();
let list_long = "list".to_string();
let shell_long = "shell".to_string();
let run_long = "run".to_string();
let rows_long = "rows".to_string();
let row_id_long = "row-id".to_string();
let dict_long = "dictionary".to_string();

let longopts = &[
    Opt::new(
        Some(help_long.clone()),
        help_long.chars().next(),
        HasArg::NoArgument,
        "help and version information",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(config_long.clone()),
        config_long.chars().next(),
        HasArg::NoArgument,
        "config file name with full path",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(list_long.clone()),
        list_long.chars().next(),
        HasArg::NoArgument,
        "list all dictionaries",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(shell_long.clone()),
        shell_long.chars().next(),
        HasArg::NoArgument,
        "run as a Lua shell",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(run_long.clone()),
        run_long.chars().next(),
        HasArg::RequiredArgument,
        "run a Lua script file",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(rows_long.clone()),
        None,
        HasArg::NoArgument,
        "query rows count in a dictionary",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(row_id_long.clone()),
        None,
        HasArg::RequiredArgument,
        "query a row information in a dictionary",
    )
    .map_err(|e| on_the_spot!(e))?,
    Opt::new(
        Some(dict_long.clone()),
        dict_long.chars().next(),
        HasArg::RequiredArgument,
        "dictionary name used to query",
    )
    .map_err(|e| on_the_spot!(e))?,
];
let opts = getopt_long(longopts).map_err(|e| on_the_spot!(e))?;

let config = {
    let mut short = config_long.clone();
    short.split_off(1);
    let config_name = if let Some(config) = opts.args.get(&short) {
        config.clone()
    } else if let Some(config) = opts.args.get(&config_long) {
        config.clone()
    } else {
        "/Users/xt/.dict/config.lua".to_string()
    };
    Config::load(&config_name)?
};

let action = loop {
    let mut short = help_long.clone();
    short.split_off(1);
    if opts.args.get(&short).is_some() || opts.args.get(&help_long).is_some() {
        usage(
            "rdict",
            "query a word information from some remote dictionaries.",
            "0.1.0",
            longopts,
        );
        break Action::Quit;
    }

    short = list_long.clone();
    short.split_off(1);
    if opts.args.get(&short).is_some() || opts.args.get(&list_long).is_some() {
        break Action::List;
    }

    short = shell_long.clone();
    short.split_off(1);
    if opts.args.get(&short).is_some() || opts.args.get(&shell_long).is_some() {
        break Action::Shell;
    }

    short = run_long.clone();
    short.split_off(1);
    if let Some(script) = opts.args.get(&short) {
        break Action::Run {script: script.clone()}
    } else if let Some(script) = opts.args.get(&run_long) {
        break Action::Run {script: script.clone()}
    }

    short = dict_long.clone();
    short.split_off(1);
    let dict_name = if let Some(dict) = opts.args.get(&short) {
        dict.clone()
    } else if let Some(dict) = opts.args.get(&dict_long) {
        dict.clone()
    } else {
        config.default_dictionary_name()?
    };

    if opts.args.get(&rows_long).is_some() {
        break Action::Rows { dict: dict_name };
    }

    if let Some(id) = opts.args.get(&row_id_long) {
        let id: u32 = id
            .parse()
            .map_err(|e: <u32 as std::str::FromStr>::Err| on_the_spot!(e))?;
        break Action::RowId {
            id,
            dict: dict_name,
        };
    }

    break Action::Query {
        words: opts.operands.iter().map(|v| v.clone()).collect(),
        dict: dict_name,
    };
};

action.do_with_config(config).await
```
In a shell, key: 
```sh
./hello -h
```

show as below:
``sh 
hello [options [args]] [operands]                                          
Options:                                                                   
    -h, --help                         help and version information        
    -c, --config                       config file name with full path     
    -l, --list                         list all dictionaries               
    -s, --shell                        run as a Lua shell.                 
    -r, --run                =Arg      run a Lua script file.              
        --rows                         query rows count in a dictionary    
        --row-id             =Arg      query a row information in a diction
ary                                                                        
    -d, --dictionary         =Arg      dictionary name used to query
``

Just so. It is very simple and useful.
