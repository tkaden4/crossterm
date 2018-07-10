//! This module contains the commands that can be used for windows systems.

use super::super::Context;
use super::{ICommand, IContextCommand};

use kernel::windows_kernel::{ansi_support, kernel};
use std::mem;
use winapi::shared::minwindef::DWORD;
use winapi::um::wincon;
use winapi::um::wincon::{CHAR_INFO, COORD, ENABLE_VIRTUAL_TERMINAL_PROCESSING, SMALL_RECT};

/// This command is used for enabling and disabling ANSI code support for windows systems,
/// For more info check: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences.
#[derive(Clone, Copy)]
pub struct EnableAnsiCommand {
    mask: DWORD,
}

impl ICommand for EnableAnsiCommand {
    fn new() -> Box<Self> {
        let key = super::generate_key();
        let command = EnableAnsiCommand {
            mask: ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        };
        Box::from(command)
    }

    fn execute(&mut self) -> bool {
        // we need to check whether we tried to enable ansi before. If we have we can just return if that had succeeded.
        if ansi_support::has_been_tried_to_enable() && ansi_support::ansi_enabled() {
            ansi_support::windows_supportable()
        } else {
            let output_handle = kernel::get_output_handle();
            let mut dw_mode: DWORD = 0;

            if !kernel::get_console_mode(&output_handle, &mut dw_mode) {
                return false;
            }

            dw_mode |= self.mask;

            kernel::set_console_mode(&output_handle, dw_mode)
        }
    }

    fn undo(&mut self) -> bool {
        if ansi_support::ansi_enabled() {
            let output_handle = kernel::get_output_handle();

            let mut dw_mode: DWORD = 0;
            if !kernel::get_console_mode(&output_handle, &mut dw_mode) {
                return false;
            }

            dw_mode &= !self.mask;
            if !kernel::set_console_mode(&output_handle, dw_mode) {
                return false;
            }

            ansi_support::set_ansi_enabled(false);
        }
        return true;
    }
}

/// This command is used for enabling and disabling raw mode for windows systems.
/// For more info check: https://docs.microsoft.com/en-us/windows/console/high-level-console-modes.
#[derive(Clone, Copy)]
pub struct EnableRawModeCommand {
    mask: DWORD,
}

impl IContextCommand for EnableRawModeCommand {
    fn new(context: &mut Context) -> (Box<EnableRawModeCommand>, i16) {
        use self::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};

        let key = super::generate_key();
        let command = EnableRawModeCommand {
            mask: ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_ECHO_INPUT,
        };
        context.register_change(Box::from(command), key);
        (Box::from(command), key)
    }

    fn execute(&mut self) -> bool {
        let input_handle = kernel::get_input_handle();

        let mut dw_mode: DWORD = 0;
        if !kernel::get_console_mode(&input_handle, &mut dw_mode) {
            return false;
        }

        dw_mode &= !self.mask;

        kernel::set_console_mode(&input_handle, dw_mode)
    }

    fn undo(&mut self) -> bool {
        let output_handle = kernel::get_output_handle();

        let mut dw_mode: DWORD = 0;
        if !kernel::get_console_mode(&output_handle, &mut dw_mode) {
            return false;
        }

        dw_mode |= self.mask;

        kernel::set_console_mode(&output_handle, dw_mode)
    }
}

/// This command is used for switching to alternate screen and back to main screen.
/// check https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes for more info
#[derive(Clone, Copy)]
pub struct ToAlternateScreenBufferCommand;

impl ICommand for ToAlternateScreenBufferCommand {
    fn new() -> Box<ToAlternateScreenBufferCommand> {
        Box::from(ToAlternateScreenBufferCommand {})
    }

    fn execute(&mut self) -> bool {
        let mut chi_buffer: [CHAR_INFO; 160] = unsafe { mem::zeroed() };

        let handle = kernel::get_output_handle();

        // create a new screen buffer to copy to.
        let new_handle = kernel::create_console_screen_buffer();

        // Make the new screen buffer the active screen buffer.
        kernel::set_active_screen_buffer(new_handle);

        // Set the source rectangle.
        let mut srct_read_rect = SMALL_RECT {
            Top: 0,
            Left: 0,
            Bottom: 1,
            Right: 79,
        };

        // The temporary buffer size is 2 rows x 80 columns.
        let coord_buffer_size = COORD { X: 2, Y: 80 };

        // The top left destination cell of the temporary buffer is
        // row 0, col 0.
        let coord_buffer_coord = COORD { X: 0, Y: 0 };

        // Copy the block from the screen buffer to the temp. buffer.
        kernel::read_console_output(
            &handle,
            &mut chi_buffer,
            coord_buffer_size,
            coord_buffer_coord,
            &mut srct_read_rect,
        );

        // Set the destination rectangle.
        let mut srct_write_rect = SMALL_RECT {
            Top: 10,
            Left: 0,
            Bottom: 11,
            Right: 19,
        };

        // Copy from the temporary buffer to the new screen buffer.
        kernel::write_console_output(
            &new_handle,
            &mut chi_buffer,
            coord_buffer_size,
            coord_buffer_coord,
            &mut srct_write_rect,
        );

        true
    }

    fn undo(&mut self) -> bool {
        let handle = kernel::get_output_handle();
        kernel::set_active_screen_buffer(handle);
        true
    }
}
