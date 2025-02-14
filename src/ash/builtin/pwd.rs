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

//! Shell 'pwd' builtin command implementation.

use std::{ ffi::CStr, process };

const SIZE: usize = libc::PATH_MAX as usize;

/// Print current working directory.
pub fn pwd(_: &Vec<*const i8>) {
    let mut buf: [i8; SIZE] = [0; SIZE];

    unsafe {
        let ret = libc::getcwd(buf.as_mut_ptr(), SIZE);

        if ret.is_null() {
            eprintln!("ash: error to get current working directory");
            process::exit(1);
        }

        let dir = CStr::from_ptr(buf.as_ptr())
            .to_string_lossy()
            .into_owned();

        println!("{}", dir);
    }
}