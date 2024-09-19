use std::{
    collections::{HashMap, VecDeque},
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};

use command::Command;
use config::Config;
use inline_colorization::*;

use lazy_static::lazy_static;
use parser::parse;
use variables::Variables;

pub mod command;
pub mod config;
pub mod default_commands;
pub mod invalid_expression;
pub mod parser;
pub mod variable;
pub mod variables;
pub mod custom_io;
pub mod parameter_documentation;

const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";

lazy_static! {
    pub static ref VARIABLES: Mutex<Variables> = Mutex::new(Variables::default());
    pub static ref BUILTIN_VARIABLE_COUNT: Mutex<usize> = Mutex::new(0);
    pub static ref HISTORY: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
    pub static ref OPERATORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref COMMANDS: Mutex<HashMap<String, Command>> = Mutex::new(HashMap::new());
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

fn main() {
    setup_default_operators();
    default_commands::DefaultCommands::register();

    let mut config = CONFIG.lock().unwrap();
    *config = Some(read_config());

    // Initialize VARIABLES early
    let variables = VARIABLES.lock().unwrap();

    // Get number of built-in variables for math later
    let mut builtin_var_count = BUILTIN_VARIABLE_COUNT.lock().unwrap();
    *builtin_var_count = variables.variables.len();

    drop(config);
    drop(builtin_var_count);
    drop(variables);

    default_commands::clear_terminal(&"".to_owned());

    let input = &mut String::new();
    loop {
        let history = HISTORY.lock().unwrap();
        let count = history.len();
        drop(history);

        input.clear();
        print!(
            "{color_blue}[{color_cyan}{}{color_blue}]> {color_reset}",
            count + 1
        );
        io::stdout().flush().unwrap();

        *input = custom_io::read_line().unwrap();
        _ = parse(input.clone());
    }
}

fn read_config() -> Config {
    let path = PathBuf::from_str("./config.toml").unwrap();
    if !path.exists() {
        let config = Config {
            time_expression: false,
        };

        _ = std::fs::write(path, toml::to_string(&config).unwrap());
        config
    } else {
        let raw = std::fs::read_to_string(path).unwrap();

        let result = toml::from_str::<Config>(&raw);

        if let Ok(result) = result {
            result
        } else {
            println!("{color_red}Config is invalid, running with default config{color_reset}");
            Config {
                time_expression: false,
            }
        }
    }
}

fn splash() {
    println!(
        "{color_blue}SmartCalc{color_reset}\n{ITALIC}Type \"help\" for a list of commands{RESET}\n",
    );
}

fn setup_default_operators() {
    let mut lock = OPERATORS.lock().unwrap();
    *lock = vec!["+", "-", "*", "/", "^", "="]
        .into_iter()
        .map(String::from)
        .collect();
}
