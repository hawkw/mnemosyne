//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//
#![crate_name = "mnemosyne"]
#![crate_type = "lib"]
#![feature(rustc_private)]
#![feature(static_recursion)]
#![feature(box_syntax, box_patterns)]

//! # Mnemosyne core
//!
//! This crate contains the core Mnemosyne programming language components.
//! This includes the mnemosyne abstract syntax tree (`semantic::ast`),
//! functions for performing semantic analysis (`semantic`), functions
//! for compiling abstract syntax trees to LLVM bytecode (`compile`), and
//! assorted utility code such as a positional reference type and a
//! `ForkTable` data structure for use as a symbol table.
//!
//! The Mnemosyne parser is contained in a separate crate in order to improve
//! compile times.

extern crate rustc;
extern crate libc;
extern crate combine;
#[macro_use] extern crate itertools;

pub mod position;
pub mod semantic;
pub mod compile;
pub mod forktable;
pub mod chars;

pub use semantic::ast;

use rustc::lib::llvm::{LLVMVersionMajor, LLVMVersionMinor};

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 1;

/// Returns the LLVM version as a String
pub fn llvm_version() -> String {
    unsafe { format!("LLVM v{}.{}", LLVMVersionMajor(), LLVMVersionMinor()) }
}

/// Returns the Mnemosyne version as a String
pub fn mnemosyne_version() -> String {
    format!("Mnemosyne v{}.{}", VERSION_MAJOR, VERSION_MINOR)
}
