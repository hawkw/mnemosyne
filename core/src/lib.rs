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

#[macro_use]
macro_rules! ice {
    ($msg:expr) => (
        panic!( "[internal error] {}\n \
                 [internal error] Something has gone horribly wrong.\n \
                 [internal error] Please contact the Mnemosyne implementors."
              , $msg)
          );
    ($fmt:expr, $($arg:tt)+) => (
        panic!( "[internal error] {}\n \
                 [internal error] Something has gone horribly wrong.\n \
                 [internal error] Please contact the Mnemosyne implementors."
              , format_args!($fmt, $($arg)+)
              )
          )
}

pub mod position;
pub mod semantic;
pub mod compile;
pub mod forktable;
pub mod chars;

pub use semantic::ast;

use rustc::lib::llvm::{LLVMVersionMajor, LLVMVersionMinor};

use std::fmt::Debug;

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

/// Wraps Option/Result with an `expect_ice()` method.
///
/// The `expect_ice()` method functions similarly to the standard library's
/// `expect()`, but with the custom Mnemosyne internal compiler error message.
pub trait ExpectICE<T> {
    fn expect_ice(self, msg: &str) -> T;
}

impl<T> ExpectICE<T> for Option<T> {
    /// Unwraps an option, yielding the content of a `Some`
    ///
    /// # Panics
    ///
    /// Panics using the Mnemosyne internal compiler error formatter
    /// if the value is a `None`, with a custom panic message
    /// provided by `msg`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mnemosyne::ExpectICE;
    /// let x = Some("value");
    /// assert_eq!(x.expect_ice("the world is ending"), "value");
    /// ```
    ///
    /// ```{.should_panic}
    /// # use mnemosyne::ExpectICE;
    /// let x: Option<&str> = None;
    /// x.expect_ice("the world is ending");
    /// ```
    #[inline]
    fn expect_ice(self, msg: &str) -> T {
        match self {
            Some(thing) => thing
          , None        => ice!(msg)
        }
    }
}
impl<T, E> ExpectICE<T> for Result<T, E>
where E: Debug {

    /// Unwraps a result, yielding the content of an `Ok`.
    ///
    /// Panics using the Mnemosyne internal compiler error formatter
    /// if the value is an `Err`, with a panic message including the
    /// passed message, and the content of the `Err`.
    ///
    /// # Examples
    /// ```{.should_panic}
    /// # use mnemosyne::ExpectICE;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.expect_ice("Testing expect");
    /// ```
    #[inline]
    fn expect_ice(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => ice!("{}: {:?}", msg, e),
        }
    }
}
