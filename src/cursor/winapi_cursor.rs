//! This is an WINAPI specific implementation for cursor related action.
//! This module is used for windows terminals that do not support ANSI escape codes.

use super::ITerminalCursor;
use kernel::windows_kernel::{cursor, kernel};
use Construct;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor;

impl Construct for WinApiCursor {
    fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor {})
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) {
        kernel::set_console_cursor_position(x as i16, y as i16);
    }

    fn pos(&self) -> (u16, u16) {
        cursor::pos()
    }

    fn move_up(&self, count: u16) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos - count);
    }

    fn move_right(&self, count: u16) {
        let (xpos, ypos) = self.pos();

        self.goto(xpos + count, ypos);
    }

    fn move_down(&self, count: u16) {
        let (xpos, ypos) = self.pos();

        self.goto(xpos, ypos + count);
    }

    fn move_left(&self, count: u16) {
        let (xpos, ypos) = self.pos();

        self.goto(xpos - count, ypos);
    }

    fn save_position(&mut self) {
        cursor::save_cursor_pos();
    }

    fn reset_position(&self) {
        cursor::reset_to_saved_position();
    }
}
