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

//! Shell 'exit' builtin command implementation.

use std::process;


/// Exit from shell.
pub fn exit(_: &Vec<*const i8>) {
    unsafe {
        let ppid = libc::getppid();

        if ppid == -1 {
            eprintln!("ash: error to get parent PID");
            process::exit(1);
        }

        let ret = libc::kill(ppid, libc::SIGTERM);

        if ret == -1 {
            eprintln!("ash: error to kill parent process");
            process::exit(1);
        }

        process::exit(0);
    }
}