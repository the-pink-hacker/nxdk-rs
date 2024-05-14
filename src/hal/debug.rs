// SPDX-License-Identifier: MIT

use cstr_core::{CStr, CString};
use nxdk_sys::hal::debug::*;

/// Prints a message to whatever debug facilities might be available.
pub fn debug_print_cstr(msg: &CStr) {
    unsafe {
        debugPrint(msg.as_ptr() as *const libc::c_char);
    }
}

/// Prints a message to whatever debug facilities might be available.
pub fn debug_print_str(msg: &str) {
    // TODO: Don't use expect()
    let cstr = CString::new(msg).expect("CString failed");
    debug_print_cstr(&cstr);
}

pub fn debug_print_number(number: impl Into<i32>) {
    let into = number.into();

    unsafe {
        debugPrintNum(into);
    }
}

pub fn debug_print_binary(number: impl Into<i32>) {
    let into = number.into();

    unsafe {
        debugPrintBinary(into);
    }
}

pub fn debug_print_hex_cstr(msg: &CStr, length: u32) {
    unsafe {
        debugPrintHex(msg.as_ptr() as *const libc::c_char, length as i32);
    }
}

pub fn debug_print_hex_str(msg: &str, length: u32) {
    // TODO: Don't use expect()
    let cstr = CString::new(msg).expect("CString failed");
    debug_print_hex_cstr(&cstr, length);
}

pub fn debug_clear_screen() {
    unsafe {
        debugClearScreen();
    }
}

pub fn debug_advance_screen() {
    unsafe {
        debugAdvanceScreen();
    }
}

pub fn debug_move_cursor(x: u32, y: u32) {
    // TODO: add bounds checking
    unsafe {
        debugMoveCursor(x as i32, y as i32);
    }
}

pub fn debug_reset_cursor() {
    unsafe {
        debugResetCursor();
    }
}
