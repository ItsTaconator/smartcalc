use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::Win32::System::Console::{GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE};

lazy_static! {
    static ref previous_mode: Mutex<Option<CONSOLE_MODE>> = Mutex::new(None);
}

pub unsafe fn enable_vt_processing() {
    let result = GetStdHandle(STD_OUTPUT_HANDLE);

    let Ok(handle) = result else {
        println!("Failed to enable VT processing");
        return;
    };


    let mut mode: CONSOLE_MODE = CONSOLE_MODE::default();

    if GetConsoleMode(handle, &mut mode).is_err() {
        println!("Failed to enable VT processing");
        return;
    }

    let mut lock = previous_mode.lock().unwrap();
    *lock = Some(mode);

    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    if SetConsoleMode(handle, mode).is_err() {
        println!("Failed to enable VT processing");
        return;
    }
}

pub unsafe fn restore_console_mode() {
    let result = GetStdHandle(STD_OUTPUT_HANDLE);

    if result.is_err() {
        println!("Failed to restore console mode");
    }

    let lock = previous_mode.lock().unwrap();
    if lock.is_some() {
        if SetConsoleMode(result.unwrap(), lock.unwrap()).is_err() {
            println!("Failed to restore console mode");
        }
    }
}