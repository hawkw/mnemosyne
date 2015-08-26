#![crate_name = "mnemosyne"]
#![crate_type = "lib"]
#![feature(rustc_private)]
#![feature(cstr_memory)]

extern crate rustc;
extern crate combine;

pub mod position;
pub mod ast;
pub mod compile;
