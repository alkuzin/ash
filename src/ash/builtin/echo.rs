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

//! Shell 'echo' builtin command implementation.

use std::ffi::CStr;


/// Print text to the terminal.
pub fn echo(argv: &Vec<*const i8>) {
    let argc = argv.len();

    let mut is_n_opt = false;
    let mut begin    = 1;

    if argc - 1 > 1 {
        let option = unsafe {
            CStr::from_ptr(argv[1])
                .to_str()
                .expect("error to convert *const i8 to &str")
        };

        match option {
            "-n" => {
                is_n_opt = true;
                begin    = 2;
            },
            _ => {}
        }

        for i in begin..argc - 1 {
            let s = unsafe {
                CStr::from_ptr(argv[i])
                    .to_str()
                    .expect("error to convert *const i8 to &str")
            };

            print!("{}", s);

            if i < argc - 2 {
                print!(" ");
            }
        }
    }

    if !is_n_opt {
        println!();
    }
}