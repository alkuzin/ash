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

use libc::{execve, fork, waitpid, WIFEXITED, WIFSIGNALED, WUNTRACED};
use std::{ffi::{CStr, CString}, io::{self, Write}, process, ptr::null};

pub struct Shell {
    prompt:     String,
    home_path:  String,
    cur_dir:    String,
}

impl Shell {
    /// Constructs a new Shell object.
    pub fn new() -> Shell {
        let username  = utils::get_username();
        let hostname  = utils::get_hostname();
        let prompt    = format!("{}@{}", username, hostname);
        let home_path = format!("/home/{}", username);
        let cur_dir   = utils::get_cur_dir().replace(&home_path, "~");

        Shell { prompt, home_path, cur_dir }
    }

    /// Run shell loop.
    pub fn run(&self) {
        let mut input = String::new();

        loop {
            // Display prompt
            print!("{}:{}$ ", self.prompt, self.cur_dir);
            let _ = io::stdout().flush();

            // Read user input
            let _ = io::stdin()
                .read_line(&mut input)
                .expect("ash: error to read input");

            // Split user input
            let mut args: Vec<*const i8> = input
                .trim()
                .split_whitespace()
                .map(|x| CString::new(x).unwrap().into_raw() as *const i8)
                .collect();

            // Last argument should be null in order to make
            // execve work correctly
            args.push(null());

            self.execute(&args);
            input.clear();
        }
    }

    /// Execute shell command.
    ///
    /// # Parameters
    /// - `argv` - given vector of arguments.
    fn execute(&self, argv: &Vec<*const i8>) {
        let pid = unsafe { fork() };

        match pid {
            // Handle child process
            0 => {
                let exec = unsafe { CStr::from_ptr(argv[0]) }
                    .to_string_lossy()
                    .into_owned();

                let path = utils::find_executable(&exec).unwrap_or_else(|| {
                    eprintln!("ash: {}: command not found", exec.trim());
                    process::exit(1);
                });

                let ret = unsafe {
                    execve(path.as_ptr(), argv.as_ptr(), null())
                };

                if ret == -1 {
                    eprintln!("ash: error to execute command");
                    process::exit(1);
                }
            },
            // Handle error
            -1 => {
                eprintln!("ash: error to create a new process");
                process::exit(1);
            },
            // Handle parent process
            _ => {
                loop {
                    // Exit status of child process
                    let mut status: i32 = 0;

                    // WUNTRACED option allows the parent to receive
                    // information about stopped child processes
                    let _ = unsafe { waitpid(pid, &mut status, WUNTRACED) };

                    // Check if the child process has exited normally
                    // or if it was terminated by a signal
                    if WIFEXITED(status) || WIFSIGNALED(status) {
                        break;
                    }
                }
            },
        }
    }
}