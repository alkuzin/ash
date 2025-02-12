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

//! Shell implementation.

mod utils;

use libc::{execve, exit, fork, waitpid, EXIT_FAILURE, WIFEXITED, WIFSIGNALED, WUNTRACED};
use utils::{get_username, get_hostname, get_cur_dir, find_executable};
use std::{ffi::{CStr, CString}, io::{self, Write}, ptr::null};

pub struct Shell {
    prompt:     String,
    home_path:  String,
    cur_dir:    String,
}

impl Shell {
    pub fn new() -> Shell {
        let username  = get_username();
        let hostname  = get_hostname();
        let prompt    = format!("{}@{}", username, hostname);
        let home_path = format!("/home/{}", username);
        let cur_dir   = get_cur_dir().replace(&home_path, "~");

        Shell { prompt, home_path, cur_dir }
    }

    pub fn run(&self) {
        let mut input = String::new();

        loop {
            print!("{}:{}$ ", self.prompt, self.cur_dir);
            let _ = io::stdout().flush();

            let _ = io::stdin()
                .read_line(&mut input)
                .expect("ash: error to read input");

            let mut args: Vec<*const i8> = input
                .trim()
                .split_whitespace()
                .map(|x| CString::new(x).unwrap().into_raw() as *const i8)
                .collect();

            args.push(null());
            input.clear();

            self.execute(&args);
        }
    }

    fn execute(&self, argv: &Vec<*const i8>) {
        let pid = unsafe { fork() };

        match pid {
            // handle child process
            0 => {
                let exec = unsafe {CStr::from_ptr(argv[0])}
                    .to_string_lossy()
                    .into_owned();

                let path = find_executable(&exec).unwrap_or_else(|| {
                    eprintln!("ash: {}: command not found", exec.trim());
                    unsafe { exit(EXIT_FAILURE) };
                });

                let ret = unsafe { execve(path.as_ptr(), argv.as_ptr(), null()) };

                if ret == -1 {
                    eprintln!("ash: {}: command not found", exec.trim());
                }
            },
            // handle error
            -1 => {
                eprintln!("ash: error to create new process");

                unsafe {
                    exit(EXIT_FAILURE);
                }
            },
            // handle parent process
            _ => {
                loop {
                    let mut status: i32 = 0;
                    let _ = unsafe { waitpid(pid, &mut status, WUNTRACED) };

                    if WIFEXITED(status) || WIFSIGNALED(status) {
                        break;
                    }
                }
            },
        }
    }
}