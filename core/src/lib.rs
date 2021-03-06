//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
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
#![crate_name = "mnemosyne"]
#![crate_type = "lib"]
#![feature(rustc_private)]
#![feature(static_recursion)]
#![feature(box_syntax, box_patterns)]

extern crate rustc;
extern crate libc;
extern crate combine;
// extern crate iron_llvm;
// extern crate llvm_sys;
#[macro_use] extern crate itertools;

use rustc::lib::llvm::{LLVMVersionMajor, LLVMVersionMinor};

// include!(concat!(env!("OUT_DIR"), "/gen.rs"));

/// Returns the Mnemosyne version as a String
pub fn mnemosyne_version() -> String {
    format!("Mnemosyne {}", env!("CARGO_PKG_VERSION"))
}

/// Returns the current LLVM version as a String
pub fn llvm_version() -> String {
    unsafe {
        format!("LLVM v{}.{}", LLVMVersionMajor(), LLVMVersionMinor())
    }
}

/// Macro for formatting an internal compiler error panic.
///
/// This should be used instead of the Rust standard library's `panic!()`
/// macro in the event of an unrecoverable internal compiler error.
#[macro_export]
macro_rules! ice {
    ($msg:expr) => (
        panic!( "[internal error] {}\n \
                 [internal error] Something has gone horribly wrong.\n \
                 [internal error] Please contact the Mnemosyne implementors.\n\
                 {}, {}"
              , $msg
              , $crate::mnemosyne_version(), $crate::llvm_version()
              )
          );
    ($fmt:expr, $($arg:tt)+) => (
        panic!( "[internal error] {}\n \
                 [internal error] Something has gone horribly wrong.\n \
                 [internal error] Please contact the Mnemosyne implementors.\n\
                 {}, {}"
              , format_args!($fmt, $($arg)+)
              , $crate::mnemosyne_version(), $crate::llvm_version()
              )
          )
}

pub mod position;
pub mod semantic;
pub mod compile;
pub mod llvm;
pub mod forktable;
pub mod chars;
pub mod errors;

pub use semantic::ast;
pub use self::errors::*;
