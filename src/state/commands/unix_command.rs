//! This module contains the commands that can be used for unix systems.

use super::IContextCommand;
use kernel::unix_kernel::terminal;
use termios::{tcsetattr, Termios, CREAD, ECHO, ICANON, TCSAFLUSH};
use Context;

const FD_STDIN: ::std::os::unix::io::RawFd = 1;

/// This command is used for switching to NoncanonicalMode.
#[derive(Clone, Copy)]
pub struct NoncanonicalModeCommand {
    key: i16,
}

impl IContextCommand for NoncanonicalModeCommand {
    fn new(context: &mut Context) -> (Box<NoncanonicalModeCommand>, i16) {
        let key = super::generate_key();
        let command = NoncanonicalModeCommand { key: key };
        context.register_change(Box::from(command), key);
        (Box::from(command), key)
    }

    fn execute(&mut self) -> bool {
        // Set noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= !ICANON;
            noncan.c_lflag &= !ECHO;
            noncan.c_lflag &= !CREAD;
            match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan) {
                Ok(_) => return true,
                Err(_) => return false,
            };
        } else {
            return false;
        }
    }

    fn undo(&mut self) -> bool {
        // Disable noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN) {
            let mut noncan = orig.clone();
            noncan.c_lflag &= ICANON;
            noncan.c_lflag &= ECHO;
            noncan.c_lflag &= CREAD;

            match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan) {
                Ok(_) => return true,
                Err(_) => return false,
            };
        } else {
            return false;
        }
    }
}

/// This command is used for enabling and disabling raw mode for the terminal.
#[derive(Clone, Copy)]
pub struct EnableRawModeCommand {
    original_mode: Option<Termios>,
    key: i16,
}

impl IContextCommand for EnableRawModeCommand {
    fn new(context: &mut Context) -> (Box<EnableRawModeCommand>, i16) {
        let key = super::generate_key();
        let command = EnableRawModeCommand {
            original_mode: None,
            key: key,
        };
        context.register_change(Box::from(command), key);
        (Box::from(command), key)
    }

    fn execute(&mut self) -> bool {
        if let Ok(original_mode) = terminal::get_terminal_mode() {
            self.original_mode = Some(original_mode);
            let mut new_mode = self.original_mode.unwrap();
            terminal::make_raw(&mut new_mode);
            terminal::set_terminal_mode(&new_mode);
            true
        } else {
            return false;
        }
    }

    fn undo(&mut self) -> bool {
        if let Ok(original_mode) = terminal::get_terminal_mode() {
            let result = terminal::set_terminal_mode(&self.original_mode.unwrap());

            match result {
                Ok(()) => true,
                Err(_) => false,
            }
        } else {
            return false;
        }
    }
}
