use std::{
    collections::VecDeque,
    io::{self, Write},
    process::exit,
    time::Instant,
};

use crossterm::{cursor, terminal::size, ExecutableCommand, QueueableCommand};
use evalexpr::eval;
use inline_colorization::*;
use regex::Regex;
use regex_split::RegexSplit;

use crate::{invalid_expression::InvalidExpression, variable::Variable, *};

pub fn parse<S: ToString>(expression: S) -> Result<(), InvalidExpression> {
    let mut expression = expression.to_string();

    let config = CONFIG.lock().unwrap();
    let time = config.as_ref().unwrap().time_expression;

    drop(config);

    let mut timer: Option<Instant> = None;

    if time {
        timer = Some(Instant::now());
    }

    match expression.as_str() {
        "\n" => {
            return Ok(());
        }
        "" => {
            exit(0);
        }
        _ => (),
    }

    _ = expression.pop();

    let result = parse_commands(&expression);
    if result {
        return Ok(());
    }

    if expression.starts_with("//") || expression.starts_with('#') {
        return parse_comment(expression);
    }

    if expression.contains('=') {
        let split: Vec<&str> = expression.split("=").collect();
        if split.len() > 2 {
            return Err(InvalidExpression {
                message: "Too many equals signs".to_owned(),
            });
        }

        let result = parse_variable_declaration(split);
        match result {
            Ok(_) => (),
            Err(err) => println!("{color_red}{err}{color_reset}"),
        }

        return Ok(());
    }

    _ = parse_line_references(&mut expression);

    if !handle_continuation(&mut expression) {
        return Ok(());
    }

    parse_variables(&mut expression)?;

    let result = calculate_and_show_result(&expression);

    if time {
        println!(
            "{ITALIC}Calculated in {} ms{RESET}",
            timer.unwrap().elapsed().as_millis()
        )
    }

    if result {
        let mut history = HISTORY.lock().unwrap();
        history.push_front(expression.to_owned());
    }

    Ok(())
}

fn calculate_and_show_result(expression: &String) -> bool {
    let result = eval(&expression);
    match result {
        Ok(result) => {
            println!("= {color_blue}{}{color_reset}", &result)
        }
        Err(_) => {
            println!("{color_red}Failed to calculate expression: check spelling and for typos, or for variable that doesn't exist{color_reset}");
            return false;
        }
    }

    true
}

fn parse_line_references(expression: &mut String) -> Result<(), InvalidExpression> {
    let history = HISTORY.lock().unwrap();

    for i in 0..history.len() {
        *expression = expression.replace(&format!("[{}]", i + 1), &history[i]);
    }

    Ok(())
}

fn parse_commands(expression: &String) -> bool {
    let commands_lock = COMMANDS.lock().unwrap();
    let commands = commands_lock.clone();
    drop(commands_lock);

    let mut split: VecDeque<&str> = expression.split(" ").collect();
    if split.len() > 0 {
        for (name, command) in commands.iter() {
            if split[0] == name
                || command.aliases.is_some()
                    && command.aliases.as_ref().unwrap().contains(&split[0])
            {
                _ = split.pop_front();
                let parameters: Vec<&str> = split.clone().into();
                let parameters = parameters.join(" ");
                (command.action)(&parameters);
                return true;
            }
        }
    }

    false
}

fn handle_continuation(expression: &mut String) -> bool {
    let expression_clone = expression.to_owned();
    let expression_trimmed = expression_clone.trim();

    let operators = OPERATORS.lock().unwrap();

    for operator in operators.iter() {
        if expression_trimmed.starts_with(operator) {
            let history = HISTORY.lock().unwrap();
            let expr = history.front();
            if expr.is_none() {
                println!(
                    "{color_red}No expressions in history to use for continuation{color_reset}"
                );
                return false;
            } else {
                *expression = format!("({}){expression}", expr.unwrap());
            }

            break;
        }
    }

    true
}

fn parse_comment(expression: String) -> Result<(), InvalidExpression> {
    let comment_prefix = if expression.starts_with('/') {
        "//"
    } else {
        "#"
    };

    let mut chars = expression.chars();

    chars.next();

    if comment_prefix.len() == 2 {
        chars.next();
    }

    let comment = format!("{comment_prefix} {}", chars.as_str().trim());

    let terminal_size = size().unwrap();

    let mut stdout = io::stdout();

    _ = stdout
        .queue(cursor::MoveUp(1))
        .unwrap()
        .queue(cursor::MoveToColumn(0));
    _ = stdout.flush();

    print!("{}", " ".repeat(terminal_size.0.into()));

    _ = stdout.execute(cursor::MoveToColumn(0));

    println!("{color_green}{comment}{color_reset}");

    Ok(())
}

fn parse_variables(expression: &mut String) -> Result<(), InvalidExpression> {
    let variables = VARIABLES.lock().unwrap();

    let mut variables_in_expression = false;

    for (name, variable) in variables.to_owned().into_iter() {
        if expression.contains(&name) {
            variables_in_expression = true;
        } else {
            if let Some(aliases) = variable.aliases {
                for alias in aliases.iter() {
                    if expression.contains(alias) {
                        variables_in_expression = true;
                    }
                }
            }
        }
    }

    if !variables_in_expression {
        return Ok(());
    }

    let operators = OPERATORS.lock().unwrap();

    let mut regex_str = operators
        .iter()
        .map(|o| format!("\\{o}|"))
        .collect::<Vec<String>>()
        .join("");
    regex_str.pop();

    let re = Regex::new(&regex_str).unwrap();
    let result: Vec<&str> = re.split_inclusive(&expression).collect();
    let mut result2: Vec<String> = result
        .clone()
        .into_iter()
        .flat_map(|f| re.split(f))
        .map(|f| f.trim().to_string())
        .collect();

    let mut j = 0usize;

    for (i, str) in result.iter().enumerate() {
        if i == result.len() - 1 {
            break;
        }

        for operator in operators.iter() {
            if str.ends_with(operator) {
                result2[i + j + 1] = operator.clone();
                j += 1;
                break;
            }
        }
    }

    for (i, str) in result2.iter_mut().enumerate() {
        // Skip operators and numbers
        if i % 2 != 0 || str.parse::<f64>().is_ok() {
            continue;
        }

        let variable = variables.get(str.clone());
        if let Some(variable) = variable {
            let value = variable.value;
            let string = value.to_string();
            *str = string;
        }
    }

    *expression = result2.join("");

    Ok(())
}

fn parse_variable_declaration(expression: Vec<&str>) -> Result<(), InvalidExpression> {
    let name = expression[0].trim();
    let value_str = expression[1].trim();
    let value = value_str.parse::<f64>();

    if value.is_err() {
        return Err(InvalidExpression {
            message: format!("Value given for variable {name} (\"{value_str}\") is not a number"),
        });
    }

    let value = value.unwrap();

    let variable = Variable::new_f64(name, value, None);

    let mut variables = VARIABLES.lock().unwrap();

    variables.add(variable);

    println!("{color_blue}{name} = {value}{color_reset}");

    Ok(())
}
