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

use clap::{Arg, App, SubCommand};

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
}
