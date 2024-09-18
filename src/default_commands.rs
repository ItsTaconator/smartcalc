//! Built-in commands
use std::io::stdout;

use crossterm::{terminal, ExecutableCommand};
use inline_colorization::*;
use variable::Variable;

use crate::{command::Command, *};

pub fn help(_: &String) {
    let commands = COMMANDS.lock().unwrap();
    println!("SmartCalc is an advanced command-line calculator with features such as variables, comments, line references, and continuation. For an example of these features, run the {color_yellow}features{color_reset} command\n\n{color_blue}Commands{color_reset}\nName (Aliases) - Help Text\n");
    for (name, command) in commands.iter() {
        let aliases = &command.aliases;

        let aliases = if let Some(aliases) = aliases {
            aliases.join(", ")
        } else {
            "".to_owned()
        };

        println!(
            "{color_yellow}{}{color_reset} ({}) - {}",
            name,
            aliases,
            command.help_text.clone().unwrap_or("No help text"),
        );
    }
}

pub fn list_variables(_: &String) {
    let variables = VARIABLES.lock().unwrap();
    let builtin_var_count = BUILTIN_VARIABLE_COUNT.lock().unwrap();
    println!("{color_cyan}{ITALIC}Built-in:{RESET}");

    let actual_variables: Vec<&Variable> = variables.variables.values().collect();

    for i in 0..*builtin_var_count {
        println!(
            "{color_yellow}{}{color_reset} - {}",
            actual_variables[i].key, actual_variables[i].value
        );
    }

    if *builtin_var_count < actual_variables.len() {
        println!("\n{color_cyan}{ITALIC}User-defined:{RESET}");
    }

    for i in *builtin_var_count..actual_variables.len() {
        println!(
            "{} - {:.25}...",
            actual_variables[i].key, actual_variables[i].value
        );
    }
}

pub fn clear_history(_: &String) {
    let mut history = HISTORY.lock().unwrap();
    history.clear();
    println!("{color_green}Cleared expression history{color_reset}");
}

pub fn clear_terminal(_: &String) {
    _ = stdout().execute(terminal::Clear(terminal::ClearType::All));
    crate::splash();
}

pub fn clear_variables(_: &String) {
    let mut variables = VARIABLES.lock().unwrap();
    let builtin_var_count = BUILTIN_VARIABLE_COUNT.lock().unwrap();
    let drain: Vec<(String, Variable)> = variables.variables.drain().collect();

    let mut i = 0usize;
    for (k, v) in drain {
        if i == *builtin_var_count {
            break;
        }

        i += 1;
        variables.variables.insert(k, v);
    }

    println!("{color_cyan}Cleared user-defined variables{color_reset}");
}

pub fn features(_: &String) {
    fn start(i: i32) {
        print!("{color_blue}[{color_cyan}{i}{color_blue}]>{color_reset}");
    }
    print!("{color_blue}Comments\n");
    print!("{color_green}// Comments look like this\n");
    print!("# They can also start with #{color_reset}\n\n");
    print!("{color_blue}Variables{color_reset}\n");
    print!("Variables are declared like this:\n");
    start(1);
    print!(" x = 25\n\n");
    print!("You can then reference them in later calculations:\n");
    start(2);
    print!(" x * 4\n{color_blue}= 100{color_reset}\n");
    print!("\n{color_blue}Continuation{color_reset}\n");
    print!("This adds the last expression to the start of the current expression if you omit the first operand:\n");
    start(3);
    print!(" / 10\n{color_blue}= 10{color_reset}\n\n");
    print!("{color_blue}Line References{color_reset}\n");
    print!("Finally, you can reference previous calculations by number:\n");
    start(4);
    print!(" [2] ^ 2\n{color_blue}= 10000{color_reset}\n\n");

    stdout().flush().unwrap();
}

/// Built-in commands
pub struct DefaultCommands;

impl DefaultCommands {
    pub fn register() {
        let help = Command {
            name: "help",
            help_text: Some("Shows this"),
            action: help,
            aliases: None,
        };

        let show_variables = Command {
            name: "showvariables",
            help_text: Some("Lists all variables"),
            action: list_variables,
            aliases: Some(vec!["listvariables", "vars", "showvars"]),
        };

        let exit = Command {
            name: "exit",
            help_text: Some("Exits SmartCalc"),
            action: |_| std::process::exit(0),
            aliases: Some(vec!["quit"]),
        };

        let clear = Command {
            name: "clear",
            help_text: Some("Clears the terminal"),
            action: clear_terminal,
            aliases: None,
        };

        let clearhistory = Command {
            name: "clearhistory",
            help_text: Some("Clears expression history"),
            action: clear_history,
            aliases: Some(vec!["clearh"]),
        };

        let clearvariables = Command {
            name: "clearvariables",
            help_text: Some("Clears user-defined variables"),
            action: clear_variables,
            aliases: Some(vec!["clearv", "clearvars"]),
        };

        let clearall = Command {
            name: "clearall",
            help_text: Some(
                "Clears everything (terminal, expression history, and user-defined variables)",
            ),
            action: |_| {
                // We don't actually care about the params so just make it an empty string
                let params = "".to_owned();

                clear_variables(&params);
                clear_history(&params);
                clear_terminal(&params);
            },
            aliases: None,
        };

        let features = Command {
            name: "features",
            help_text: Some(
                "Shows you some of the more unique features that SmartCalc has to offer",
            ),
            action: features,
            aliases: None,
        };

        let mut commands = COMMANDS.lock().unwrap();
        commands.insert(help.name.to_owned(), help);
        commands.insert(show_variables.name.to_owned(), show_variables);
        commands.insert(exit.name.to_owned(), exit);
        commands.insert(clear.name.to_owned(), clear);
        commands.insert(clearhistory.name.to_owned(), clearhistory);
        commands.insert(clearvariables.name.to_owned(), clearvariables);
        commands.insert(clearall.name.to_owned(), clearall);
        commands.insert(features.name.to_owned(), features);
    }
}
