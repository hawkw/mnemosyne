// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Get the output from running `llvm-config` with the given argument.
/// Taken from `llvm-sys`
fn llvm_config(arg: &str) -> String {
    String::from_utf8(
        Command::new("llvm-config")
            .arg(arg)
            .output()
            .unwrap_or_else(|e|
                panic!("Couldn't execute llvm-config. Error: {}", e))
            .stdout
        ).ok()
         .expect("llvm-config output was not UTF-8.")
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");

    File::create(&dest_path)
        .and_then(|ref mut f| write!(f,
            "/// Returns the LLVM version as a String\n \
            pub fn llvm_version() -> String {{\n \
            \tString::from(\"{}\")\n \
            }}\n \
            ", &llvm_config("--version")
            ))
        .unwrap();
}
