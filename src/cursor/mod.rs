//! This trait defines the actions that can be preformed with the terminal cursor.
//! This trait can be implemented so that an concrete implementation of the ITerminalCursor can for fill
//! the wishes to work on an specific platform.
//!
//! ## For example:
//!
//! This trait is implemented to work with WINAPI (Windows specific) and ANSI (Unix specific),
//! so that the cursor related actions can be preformed on both unix and windows systems.
//!

mod ansi_cursor;
mod cursor;
#[cfg(target_os = "windows")]
mod winapi_cursor;

use self::ansi_cursor::AnsiCursor;
#[cfg(target_os = "windows")]
use self::winapi_cursor::WinApiCursor;

pub use self::cursor::{cursor, TerminalCursor};

///! This trait defines the actions that can be preformed with the terminal cursor.
///! This trait can be implemented so that an concrete implementation of the ITerminalCursor can forfill
///! the wishes to work on an specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
///! so that cursor related actions can be preformed on both unix and windows systems.
pub trait ITerminalCursor {
    /// Goto some location (x,y) in the terminal.
    fn goto(&self, x: u16, y: u16);
    /// Get the location (x,y) of the current curor in the terminal
    fn pos(&self) -> (u16, u16);
    /// Move cursor n times up
    fn move_up(&self, count: u16);
    /// Move the cursor `n` times to the right.
    fn move_right(&self, count: u16);
    /// Move the cursor `n` times down.
    fn move_down(&self, count: u16);
    /// Move the cursor `n` times left.
    fn move_left(&self, count: u16);
    /// Save cursor position for recall later. Note that this position is stored program based not per instance of the cursor struct.
    fn save_position(&mut self);
    /// Return to saved cursor position
    fn reset_position(&self);
}
