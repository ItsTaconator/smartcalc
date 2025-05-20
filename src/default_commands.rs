//! Built-in commands
use std::io::stdout;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use custom_io::mark_special;
use inline_colorization::*;
use parameter_documentation::ParameterDocumentation;
use radix_fmt::radix;
use variable::Variable;

use crate::{command::Command, *};

/// Prints a general help message, or help for a specific command
pub fn help(command_name: &String) {
    let commands = COMMANDS.lock().unwrap();
    if command_name.len() > 0 {
        let mut command: Option<&Command> = None;
        for (name, cmd) in commands.iter() {
            if command_name == name {
                command = Some(cmd);
                break;
            }

            if let Some(aliases) = &cmd.aliases {
                if aliases.contains(&command_name.as_str()) {
                    command = Some(cmd);
                    break;
                }
            }
        }

        if command.is_none() {
            println!("{color_red}Command \"{command_name}\" not found{RESET}");
            return;
        }

        let command = command.unwrap();

        println!(
            "{color_yellow}{style_bold}{}{RESET} - {}",
            command.name,
            command.help_text.unwrap_or("No help text")
        );

        if let Some(aliases) = &command.aliases {
            println!("Aliases: {}", aliases.join(","));
        }

        if let Some(parameters) = &command.parameter_documentation {
            println!("\n{color_blue}Parameters{RESET}");
            for (name, desc, expected_type) in parameters.clone().into_iter() {
                println!("{color_magenta}{name}{RESET} - {desc} - Should be {color_magenta}{expected_type}{RESET}");
            }
        }

        return;
    }

    println!("SmartCalc is an advanced command-line calculator with features such as variables, comments, line references, and continuation.\nFor an example of these features, run the {color_yellow}features{color_reset} command\n\n{color_blue}Commands{color_reset}\nName (Aliases) - Help Text\n");
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

/// Shows user all built-in and user-defined variables and their values
pub fn show_variables(_: &String) {
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

/// Shows the user the expression history
pub fn show_history(_: &String) {
    let history = HISTORY.lock().unwrap();
    let fixed: Vec<String> = history.iter().map(|elem| elem.replace("\n", "")).collect();
    println!("{color_blue}History{RESET}\n{}", fixed.join("\n"));
    if fixed.len() == 0 {
        _ = stdout().queue(cursor::MoveUp(1));
        println!("{ITALIC}Very quiet here{RESET}");
    }
}

/// Exits cleanly
pub fn exit(_: &String) {
    mark_special("bye", "");

    // Restore previous console mode on Windows
    #[cfg(windows)]
    crate::windows::restore_console_mode();

    std::process::exit(0);
}

/// Clears expression history
pub fn clear_history(_: &String) {
    let mut history = HISTORY.lock().unwrap();
    history.clear();
    println!("{color_green}Cleared expression history{color_reset}");
}

/// Clears terminal and displays splash again
pub fn clear_terminal(_: &String) {
    _ = stdout()
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap()
        .execute(cursor::MoveTo(0, 0));

    splash();
}

/// Clears all user-defined variables
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

/// Shows off Smartcalc's features
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
    println!("You can even do a continuation after a continuation:");
    start(4);
    print!(" % 5\n{color_blue}= 0{color_reset}\n\n");
    print!("{color_blue}Line References{color_reset}\n");
    print!("Finally, you can reference previous calculations by number:\n");
    start(5);
    print!(" [2] ^ 2\n{color_blue}= 10000{color_reset}\n\n");

    stdout().flush().unwrap();
}

/// Number converter for binary, octal, decimal, and hexadecimal numbers
pub fn convert(number: &String) {
    let mut number = number.to_lowercase();

    let hex: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'];

    if number.len() == 0 {
        println!("{color_red}Number to convert not specified{RESET}");
        return;
    }

    let number_base = if number.starts_with("0b") {
        2
    } else if number.starts_with("0o") {
        8
    } else if number.starts_with("0x") {
        16
    } else if number.chars().any(|c| hex.contains(&c)) {
        println!("Assuming base is 16");
        16
    } else {
        println!("Assuming base is 10");
        10
    };

    if number_base != 10 {
        number = number.chars().skip(2).collect();
    }

    let result = isize::from_str_radix(&number, number_base);

    if result.is_err() {
        println!("{color_red}Could not parse number{RESET}");
        return;
    }

    let actual_number = result.unwrap();

    if number_base != 2 {
        println!("{color_blue}Binary:{RESET} {}", radix(actual_number, 2));
    }

    if number_base != 8 {
        println!("{color_blue}Octal:{RESET} {}", radix(actual_number, 8));
    }

    if number_base != 10 {
        println!("{color_blue}Decimal:{RESET} {}", radix(actual_number, 10));
    }

    if number_base != 16 {
        println!(
            "{color_blue}Hexadecimal:{RESET} {}",
            radix(actual_number, 16).to_string().to_uppercase()
        );
    }

    println!();
}

/// Built-in commands
pub struct DefaultCommands;

impl DefaultCommands {
    /// Registers all built-in commands
    pub fn register() {
        let help = Command {
            name: "help",
            help_text: Some(
                "Shows all commands, or info about a specific command if followed by its name",
            ),
            action: help,
            aliases: None,
            parameter_documentation: Some(ParameterDocumentation::new(
                vec!["command"],
                vec!["(Optional) Command to get help for, lists all commands if not specified"],
                vec!["String or Nothing"],
            )),
        };

        let show_variables = Command {
            name: "showvariables",
            help_text: Some("Lists all variables"),
            action: show_variables,
            aliases: Some(vec!["listvariables", "vars", "showvars", "showv"]),
            parameter_documentation: None,
        };

        let show_history = Command {
            name: "showhistory",
            help_text: Some("Shows expression history"),
            action: show_history,
            aliases: Some(vec!["history", "showh"]),
            parameter_documentation: None,
        };

        let exit = Command {
            name: "exit",
            help_text: Some("Exits SmartCalc"),
            action: exit,
            aliases: Some(vec!["quit"]),
            parameter_documentation: None,
        };

        let clear = Command {
            name: "clear",
            help_text: Some("Clears the terminal"),
            action: clear_terminal,
            aliases: None,
            parameter_documentation: None,
        };

        let clearhistory = Command {
            name: "clearhistory",
            help_text: Some("Clears expression history"),
            action: clear_history,
            aliases: Some(vec!["clearh"]),
            parameter_documentation: None,
        };

        let clearvariables = Command {
            name: "clearvariables",
            help_text: Some("Clears user-defined variables"),
            action: clear_variables,
            aliases: Some(vec!["clearv", "clearvars"]),
            parameter_documentation: None,
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
            aliases: Some(vec!["cleara"]),
            parameter_documentation: None,
        };

        let features = Command {
            name: "features",
            help_text: Some(
                "Shows you some of the more unique features that SmartCalc has to offer",
            ),
            action: features,
            aliases: None,
            parameter_documentation: None,
        };

        let convert = Command {
            name: "convert",
            help_text: Some(
                "Converts between number bases.\nNumber base can be specified by prefixing number with 0b for binary, 0o for octal, 0d for decimal, and 0x for hexadecimal.\nIf not specified, SmartCalc will guess"
            ),
            action: convert,
            aliases: None,
            parameter_documentation: Some(ParameterDocumentation::new(vec!["number"], vec!["Number to convert"], vec!["Integer"]))
        };

        let mut commands = COMMANDS.lock().unwrap();
        commands.insert(help.name.to_owned(), help);
        commands.insert(show_variables.name.to_owned(), show_variables);
        commands.insert(show_history.name.to_owned(), show_history);
        commands.insert(exit.name.to_owned(), exit);
        commands.insert(clear.name.to_owned(), clear);
        commands.insert(clearhistory.name.to_owned(), clearhistory);
        commands.insert(clearvariables.name.to_owned(), clearvariables);
        commands.insert(clearall.name.to_owned(), clearall);
        commands.insert(features.name.to_owned(), features);
        commands.insert(convert.name.to_owned(), convert);
    }
}
