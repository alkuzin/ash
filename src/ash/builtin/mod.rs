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

//! Shell builtin commands implementation.

mod clear;
mod echo;
mod exit;
mod pwd;
mod cd;

pub type BuiltinFuncPtr = fn(&Vec<*const i8>) -> ();

struct Builtin {
    pub name: &'static str,
    pub func: BuiltinFuncPtr,
}

const BUILTINS_COUNT: usize = 5;

static BUILTINS_TABLE: [Builtin;BUILTINS_COUNT] = [
    Builtin { name: "cd",    func: cd::cd       },
    Builtin { name: "clear", func: clear::clear },
    Builtin { name: "exit",  func: exit::exit   },
    Builtin { name: "pwd",   func: pwd::pwd     },
    Builtin { name: "echo",  func: echo::echo   },
];

pub fn get_builtin(command: &str) -> Option<BuiltinFuncPtr> {
    for builtin in &BUILTINS_TABLE {
        if command == builtin.name {
            return Some(builtin.func);
        }
    }
    None
}