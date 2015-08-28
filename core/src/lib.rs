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
#![feature(cstr_memory)]

extern crate rustc;
extern crate combine;
extern crate seax_util as seax;

pub mod position;
pub mod ast;
pub mod compile;

use rustc::lib::llvm::{LLVMVersionMajor, LLVMVersionMinor}

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 1;

pub fn llvm_version() -> String {
    format!("LLVM v{}.{}", LLVMVersionMajor(), LLVMVersionMinor())
}

pub fn mnemosyne_version() -> String {
    format!("Mnemosyne v{}.{}", VERSION_MAJOR, VERSION_MINOR)
}
