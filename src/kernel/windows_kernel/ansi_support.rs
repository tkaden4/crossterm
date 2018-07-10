//! This module handles the enabling `ANSI escape codes` for windows terminals.

use state::commands::ICommand;
use Context;

static mut HAS_BEEN_TRYED_TO_ENABLE: bool = false;
static mut IS_ANSI_ON_WINDOWS_ENABLED: Option<bool> = None;
static mut DOES_WINDOWS_SUPPORT_ANSI: Option<bool> = None;

/// Try enable `ANSI escape codes` and return the result.
pub fn try_enable_ansi_support() -> bool {
    use state::commands::win_commands::EnableAnsiCommand;
    let mut command = EnableAnsiCommand::new();
    let success = command.execute();

    set_is_windows_ansi_supportable(success);
    set_ansi_enabled(success);
    set_has_been_tried_to_enable(true);

    success
}

/// Get whether ansi has been enabled.
pub fn ansi_enabled() -> bool {
    unsafe { IS_ANSI_ON_WINDOWS_ENABLED.unwrap_or(false) }
}

/// Get whether windows supports ansi
pub fn windows_supportable() -> bool {
    unsafe { DOES_WINDOWS_SUPPORT_ANSI.unwrap_or(false) }
}

/// Get whether ansi has been tried to enable before.
pub fn has_been_tried_to_enable() -> bool {
    unsafe { HAS_BEEN_TRYED_TO_ENABLE }
}

/// Set the is ansi escape property enabled or disabled. So whe can determine if the ansi escape codes are enabled.
pub fn set_ansi_enabled(is_enabled: bool) {
    unsafe {
        IS_ANSI_ON_WINDOWS_ENABLED = Some(is_enabled);
    }
}

/// Set the is_windows_ansi_supportable property. So whe can determine whether windows supports ansi.
fn set_is_windows_ansi_supportable(is_enabled: bool) {
    unsafe {
        DOES_WINDOWS_SUPPORT_ANSI = Some(is_enabled);
    }
}

/// Set the has_been_tried_to_enable property. So we can determine whether ansi has been tried to enable before.
fn set_has_been_tried_to_enable(has_been_tried: bool) {
    unsafe {
        HAS_BEEN_TRYED_TO_ENABLE = has_been_tried;
    }
}
