use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, position},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use inline_colorization::*;

use crate::*;

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    let (original_x, _) = position()?;
    _ = stdout().execute(cursor::SavePosition);
    _ = enable_raw_mode();

    let mut current_history_entry = -1;

    while let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        let (x, _) = position()?;
        let mut clear_line = |flush| -> io::Result<()> {
            if line.len() != 0 {
                let len = line.len();
                line.clear();
                stdout().queue(cursor::MoveLeft(len as u16))?;
                print!("{}", " ".repeat(len.into()));
                stdout().queue(cursor::MoveLeft(len as u16))?;
                if flush {
                    stdout().flush()?;
                }
            }

            Ok(())
        };

        match code {
            KeyCode::Enter => {
                _ = disable_raw_mode();
                println!();
                line.push('\n');
                break;
            }
            // Grab last expression from history and replace current one with it
            KeyCode::Up => {
                let history = HISTORY.lock().unwrap();

                if history.len() > 0 {
                    current_history_entry += 1;
                    if history.len() <= (current_history_entry as usize) {
                        current_history_entry = history.len() as isize;
                        continue;
                    }
                    
                    clear_line(false)?;

                    let mut entry = history
                        .get(current_history_entry as usize)
                        .unwrap()
                        .to_owned();

                    entry = entry.trim().to_owned();

                    print!("{}", &entry);

                    line = entry;

                    _ = stdout().flush()?;
                }
            }
            // Move cursor left (within bounds)
            KeyCode::Left => {
                if original_x != x {
                    stdout().execute(cursor::MoveLeft(1))?;
                }
            }
            // Move cursor right (within bounds)
            KeyCode::Right => {
                let prompt_length = HISTORY.lock().unwrap().len() + 5;
                if (x as usize) < (line.len() + prompt_length) {
                    stdout().execute(cursor::MoveRight(1))?;
                }
            }
            // Go down in history or clear input
            KeyCode::Down => {
                let history = HISTORY.lock().unwrap();

                if history.len() > 0 {
                    current_history_entry -= 1;
                    if current_history_entry < 0 {
                        current_history_entry = -1;
                        clear_line(true)?;
                        continue;
                    }

                    clear_line(false)?;

                    let entry = history
                        .get(current_history_entry as usize)
                        .unwrap()
                        .trim().to_owned();

                    print!("{}", &entry);

                    line = entry;

                    _ = stdout().flush()?;
                } else {
                    clear_line(true)?;
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                if line.len() != 0 {
                    _ = line.pop();
                    _ = stdout().queue(cursor::MoveLeft(1));
                    print!(" ");
                    _ = stdout().execute(cursor::MoveLeft(1));
                }
            }
            KeyCode::Char(c) => {
                // CTRL
                if modifiers == KeyModifiers::CONTROL {
                    match c {
                        'c' | 'd' => {
                            _ = disable_raw_mode();
                            println!();
                            default_commands::exit(&"".to_owned());
                        }
                        _ => ()
                    }
                } else {
                    line.push(c);
                    print!("{c}");
                    _ = stdout().flush()?;
                }
            }
            _ => {}
        }
    }

    Ok(line)
}

/// This is a helper function that replaces the `#` in `[#]> expression` with `marker`
pub fn mark_special(marker: &str, expression: &str) {
    _ = stdout()
        .queue(cursor::MoveUp(1))
        .unwrap()
        .queue(terminal::Clear(terminal::ClearType::CurrentLine));

    println!("{color_blue}[{color_cyan}{marker}{color_blue}]> {RESET}{expression}");
}
