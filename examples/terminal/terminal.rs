//!
//! Terminal Examples
//!

extern crate crossterm;

use crossterm::cursor;
use crossterm::terminal::{terminal, ClearType, Terminal};

fn print_test_data() {
    for i in 0..100 {
        println!("Test data to test terminal: {}", i);
    }
}

/// Clear all lines in terminal | demonstration
pub fn clear_all_lines() {
    // Get terminal
    let mut terminal = terminal();

    print_test_data();

    // Clear all lines in terminal;
    terminal.clear(ClearType::All);
}

/// Clear all lines from cursor position X:4, Y:4 down | demonstration
pub fn clear_from_cursor_down() {
    // Get terminal
    let mut terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor::cursor().goto(4, 8);

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorDown);
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
pub fn clear_from_cursor_up() {
    // Get terminal
    let mut terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor::cursor().goto(4, 4);

    // Clear all cells from current cursor position down.
    terminal.clear(ClearType::FromCursorUp);
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
pub fn clear_current_line() {
    // Get terminal
    let mut terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor::cursor().goto(4, 4);

    // Clear current line cells.
    terminal.clear(ClearType::CurrentLine);
}

/// Clear all lines from cursor position X:4, Y:7 up | demonstration
pub fn clear_until_new_line() {
    // Get terminal
    let mut terminal = terminal();

    print_test_data();

    // Set terminal cursor position (see example for more info).
    cursor::cursor().goto(4, 20);

    // Clear all the cells until next line.
    terminal.clear(ClearType::UntilNewLine);
}

/// Print the the current terminal size | demonstration.
pub fn print_terminal_size() {
    // Get terminal
    let mut terminal = terminal();
    // Get terminal size
    let terminal_size = terminal.terminal_size();
    // Print results
    print!("X: {}, y: {}", terminal_size.0, terminal_size.1);
}

/// Set the terminal size to width 10, height: 10 | demonstration.
pub fn set_terminal_size() {
    let mut terminal = terminal();

    terminal.set_size(10, 10);
}

/// Scroll down 10 lines | demonstration.
pub fn scroll_down() {
    print_test_data();
    // Get terminal
    let mut terminal = terminal();
    // Scroll down 10 lines.
    terminal.scroll_down(10);
}

/// Scroll down 10 lines | demonstration.
pub fn scroll_up() {
    print_test_data();

    // Get terminal
    let mut terminal = terminal();
    // Scroll up 10 lines.
    terminal.scroll_up(10);
}

/// Resize the terminal to X: 10, Y: 10 | demonstration.
pub fn resize_terminal() {
    // Get terminal
    let mut terminal = terminal();
    // Get terminal size
    terminal.set_size(10, 10);
}
