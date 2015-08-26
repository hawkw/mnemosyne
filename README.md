Mnemosyne
=========

A functional systems programming language with compile-time memory management.

This repository contains `mnemosyne`, the Mnemosyne compiler implemented as a library, and Manganese (`mn`), a command-line application for compiling Mnemosyne programs. These codebases are separated so that the Mnemosyne compiler can be embedded in other applications.

**Note**: Mnemosyne is currently highly experimental and in the early phases of development. Input and feedback from users is welcomed at all stages during the design and implementation process. However, please note that as Mnemosyne continues to grow and evolve, no guarantees are made of stability or backwards compatibility. Until the release of v1.0 of Mnemosyne, code may break or change in behaviour with little or no warning.

Mnemosyne is being developed and implemented by [Hawk Weisman](http://hawkweisman.me) as senior thesis research at Allegeny College.

Instructions
------------

### Building Mnemosyne

Mnemosyne is currently implemented using the [Rust](http://www.rust-lang.org) programming language. To build Mnemosyne, use [Cargo](http://doc.crates.io/guide.html), the Rust build automation tool.

Note that Mnemosyne currently requires features available only on the nightly Rust release channel. Thus, it will not compile against stable or beta builds of Rust at this time. When installing Rust, ensure you have selected the latest Rust nightly. This is necessary as Mnemosyne relies on `rustc`'s internal LLVM bindings to interact with the LLVM backend, and access to `rustc` internals is available only on nightly Rust at this time.

Build Mnemosyne with the command `$ cargo build --release`  from the root directory of this repository. This will build a debug executable of Manganese, the Mnemosyne compiler. This executable will be output to `target/release/mn`. Alternatively, the command `$ cargo build` without the `--release` option will generate a less highly optimised debug executable. This is useful for Mnemosyne development and testing purposes.

The command `$ cargo test` will run all of the tests for Manganese, and all the Mnemosyne integration tests. To run tests for the Mnemosyne Core and Parser modules as well, run the commands `$ cargo test -p mnemosyne` and `$cargo test -p mnemosyne-parser`, respectively.
