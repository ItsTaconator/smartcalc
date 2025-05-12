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
            KeyCode::Up => {
                let history = HISTORY.lock().unwrap();

                if history.len() > 0 {
                    current_history_entry += 1;

                    _ = stdout()
                        .queue(cursor::RestorePosition)?
                        .queue(cursor::SavePosition)?
                        .flush()
                        .unwrap();

                    let (width, _) = terminal::size().unwrap();

                    print!("{}", "".repeat((width - x).into()));

                    _ = stdout().queue(cursor::RestorePosition)?.flush().unwrap();

                    let mut entry = history
                        .get(current_history_entry as usize)
                        .unwrap()
                        .to_owned();
                    entry.pop();

                    print!("{}", &entry);

                    line = entry;

                    _ = stdout().flush().unwrap();
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                if x != position().unwrap().0 {
                    _ = line.pop();
                    _ = stdout().queue(cursor::MoveLeft(1));
                    print!(" ");
                    _ = stdout().execute(cursor::MoveLeft(1));
                }
            }
            KeyCode::Char(c) => {
                if modifiers == KeyModifiers::CONTROL && (c == 'c' || c == 'd') {
                    _ = disable_raw_mode();
                    println!("");
                    default_commands::exit(&"".to_owned());
                }

                line.push(c);
                print!("{c}");
                _ = stdout().flush().unwrap();
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
        .queue(terminal::Clear(crossterm::terminal::ClearType::CurrentLine));

    println!("{color_blue}[{color_cyan}{marker}{color_blue}]> {RESET}{expression}");
}
