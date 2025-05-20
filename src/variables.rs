use core::f64;
use std::io::{self, Write};

use crossterm::{cursor, terminal, QueueableCommand};
use linked_hash_map::LinkedHashMap;

use crate::variable::Variable;

#[derive(Clone)]
pub struct Variables {
    pub variables: LinkedHashMap<String, Variable>,
    pub builtin_variable_count: usize,
}

impl IntoIterator for Variables {
    type Item = (String, Variable);

    type IntoIter = linked_hash_map::IntoIter<String, Variable>;

    fn into_iter(self) -> Self::IntoIter {
        self.variables.into_iter()
    }
}

impl Default for Variables {
    fn default() -> Self {
        let mut variables = LinkedHashMap::<String, Variable>::new();
        Variables::add_basic_constants(&mut variables);

        let builtin_variable_count = variables.len();

        Self {
            variables,
            builtin_variable_count,
        }
    }
}

impl Variables {
    pub(crate) fn add_basic_constants(variables: &mut LinkedHashMap<String, Variable>) {
        // Euler's number
        variables.insert(
            "e".to_owned(),
            Variable::new_f64("e", f64::consts::E, Some(vec!["euler".to_string()])),
        );

        // Euler's constant
        variables.insert(
            "γ".to_owned(),
            Variable::new_f64("γ", f64::exp(1.0), Some(vec!["ℇ".to_string()])),
        );

        // Pi
        variables.insert(
            "pi".to_owned(),
            Variable::new_f64("pi", f64::consts::PI, Some(vec!["π".to_string()])),
        );

        _ = io::stdout()
            .queue(cursor::MoveUp(1))
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::CurrentLine));
        io::stdout().flush().unwrap();
    }

    pub fn add(&mut self, variable: Variable) {
        let value = self.variables.get_mut(&variable.key.to_string());
        if let Some(value) = value {
            *value = variable;
        } else {
            self.variables.insert(variable.key.to_string(), variable);
        }
    }

    pub fn get<S: ToString>(&self, key: S) -> Option<Variable> {
        let key = key.to_string();
        for (k, v) in self.variables.iter() {
            if *k == key {
                return Some(v.clone());
            }

            if let Some(aliases) = &v.aliases {
                if aliases.contains(&key) {
                    return Some(v.clone());
                }
            }
        }

        None
    }

    pub fn remove<S: ToString>(mut self, key: S) {
        let key = key.to_string();
        for (k, v) in self.variables.clone().iter() {
            if *k == key || (v.aliases.is_some() && v.aliases.as_ref().unwrap().contains(k)) {
                self.variables.remove(k);
                return;
            }
        }
    }
}
