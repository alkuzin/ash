// Simple shell for Linux.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Auxilar functions.

use libc::{gethostname, getlogin, getcwd};
use std::{ffi::{CStr, CString}, env, fs};

/// Get user login.
///
/// # Returns
///
/// - String representation of username - in case of success.
/// - `"user"` - in case of failure.
pub fn get_username() -> String {
    let login = unsafe { getlogin() };

    if login.is_null() {
        return "user".to_string();
    }

    unsafe {
        CStr::from_ptr(login).to_string_lossy().into_owned()
    }
}

/// Get hostname.
///
/// # Returns
///
/// - String representation of hostname - in case of success.
/// - `"-"` - in case of failure.
pub fn get_hostname() -> String {
    const SIZE: usize = 256;
    let mut name: [i8; SIZE] = [0; SIZE];

    let ret = unsafe { gethostname(name.as_mut_ptr(), SIZE) };

    if ret == -1 {
        return String::from("-");
    }

    unsafe {
        CStr::from_ptr(name.as_mut_ptr()).to_string_lossy().into_owned()
    }
}

/// Get current directory.
///
/// # Returns
///
/// - String representation of current directory - in case of success.
/// - `"?"` - in case of failure.
pub fn get_cur_dir() -> String {
    const SIZE: usize = 4096;
    let mut cwd: [i8; SIZE] = [0; SIZE];

    let ret = unsafe { getcwd(cwd.as_mut_ptr(), SIZE) };

    if ret.is_null() {
        return "?".to_string();
    }

    unsafe {
        CStr::from_ptr(cwd.as_mut_ptr()).to_string_lossy().into_owned()
    }
}

/// Find executable path.
///
/// # Parameters
/// - `command` - given command/potential executable name.
///
/// # Returns
/// - CString representation of executable full path - in case of success.
/// - `None` - in case of failure.
pub fn find_executable(command: &str) -> Option<CString> {
    let path = env::var("PATH")
        .expect("find_executable: error to get PATH variable");

    for dir in path.split(":") {
        let exec_path = format!("{}/{}", dir, command);

        match fs::exists(&exec_path) {
            Ok(res) => {
                if res {
                    return Some(CString::new(exec_path).unwrap());
                }
            },
            Err(_) => {}
        }
    }
    None
}