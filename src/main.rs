//
// The Manganese Mnemosyne Compilation System
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//
extern crate clap;
extern crate mnemosyne;
extern crate mnemosyne_parser as parser;

use clap::{Arg, App, SubCommand};

use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use mnemosyne::ast;
use mnemosyne::ast::Node;
use mnemosyne::errors::UnwrapICE;

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 1;

fn main() {
    let matches = App::new("Manganese")
        .version(&format!("v{}.{} for {} ({})"
                , VERSION_MAJOR
                , VERSION_MINOR
                , mnemosyne::mnemosyne_version()
                , mnemosyne::llvm_version()
            ))
        .author("Hawk Weisman <hi@hawkweisman.me>")
        .about("[Mn] Manganese: The Mnemosyne Compilation System")
        .args_from_usage(
            "<INPUT> 'Source code file to compile'
             -d, --debug 'Display debugging information'")
        .get_matches();

    let path = matches.value_of("INPUT")
                      .map(PathBuf::from)
                      .unwrap();

    let code = File::open(&path)
        .map_err(|error    | String::from(error.description()) )
        .and_then(|mut file| {
                let mut s = String::new();
                file.read_to_string(&mut s)
                    .map_err(|error| String::from(error.description()) )
                    .map(|_| s)
            })
        .unwrap();

     let ast = parser::parse_module(code.as_ref())
                     .unwrap();

    for node in ast { println!("{}", (*node).to_sexpr(0)) }
}
