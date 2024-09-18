use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, position},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use crate::HISTORY;

pub fn read_line() -> std::io::Result<String> {
    let mut line = String::new();
    let (x, _) = cursor::position().unwrap();
    _ = stdout().execute(cursor::SavePosition);
    _ = enable_raw_mode();

    let mut current_history_entry = -1;

    while let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
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
                    std::process::exit(0);
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
