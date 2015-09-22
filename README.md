Mnemosyne
=========

[![Travis build status](https://img.shields.io/travis/hawkw/mnemosyne/master.svg?style=flat-square)](https://travis-ci.org/hawkw/mnemosyne)
[![Coverage](https://img.shields.io/codecov/c/github/mnemosyne/master.svg?style=flat-square)](http://codecov.io/github/hawkw/mnemosyne?branch=master)
[![GitHub license](https://img.shields.io/github/license/hawkw/mnemosyne.svg?style=flat-square)](https://github.com/hawkw/mnemosyne/blob/master/LICENSE)
[![GitHub release](https://img.shields.io/github/release/hawkw/mnemosyne.svg?style=flat-square)](https://github.com/hawkw/mnemosyne/releases)

A functional systems programming language with compile-time memory management.

This repository contains `mnemosyne`, the Mnemosyne compiler implemented as a library, and Manganese (`mn`), a command-line application for compiling Mnemosyne programs. These codebases are separated so that the Mnemosyne compiler can be embedded in other applications.

**Note**: Mnemosyne is currently highly experimental and in the early phases of development. Input and feedback from users is welcomed at all stages during the design and implementation process. However, please note that as Mnemosyne continues to grow and evolve, no guarantees are made of stability or backwards compatibility. Until the release of v1.0 of Mnemosyne, code may break or change in behaviour with little or no warning.

Mnemosyne is being developed and implemented by [Hawk Weisman](http://hawkweisman.me) as [senior thesis research](https://github.com/hawkw/senior-thesis) at Allegheny College.


About Mnemosyne
---------------

Mnemosyne's main goal is to provide programmers with the safety and elegance of modern functional programming languages while still being a viable choice for high-performance and low-level programming tasks. Primary to this objective is the use of automatic compile-time memory management through ownership types and lifetime analysis, which gives programmers the tools to write high-performance systems and applications that guarantee memory safety without garbage collection. A general focus is placed on the use of zero-cost abstractions and on resolving costly operations at compile-time rather than at runtime.

Mnemosyne is a statically-typed functional programming language. Its syntax is primarily inspired by the LISP family of languages, particularly Scheme, Typed Racket, and to a lesser extent, Clojure. Its semantics and compiler design, however, are influenced more by Haskell and languages in the ML family, Rust, and C++.

Please note that this Mnemosyne implementation is intended primarily as a prototype or technology demonstration. Many major planned features have not yet been implemented.

Instructions
------------

### Building Mnemosyne

Mnemosyne is currently implemented using the [Rust](http://www.rust-lang.org) programming language. To build Mnemosyne, use [Cargo](http://doc.crates.io/guide.html), the Rust build automation tool.

Note that Mnemosyne currently requires features available only on the nightly Rust release channel. Thus, it will not compile against stable or beta builds of Rust at this time. When installing Rust, ensure you have selected the latest Rust nightly. This is necessary as Mnemosyne relies on `rustc`'s internal LLVM bindings to interact with the LLVM backend, and access to `rustc` internals is available only on nightly Rust at this time.

Build Mnemosyne with the command `$ cargo build --release`  from the root directory of this repository. This will build a debug executable of Manganese, the Mnemosyne compiler. This executable will be output to `target/release/mn`. Alternatively, the command `$ cargo build` without the `--release` option will generate a less highly optimised debug executable. This is useful for Mnemosyne development and testing purposes.

The command `$ cargo test` will run all of the tests for Manganese, and all the Mnemosyne integration tests. To run tests for the Mnemosyne Core and Parser modules as well, run the commands `$ cargo test -p mnemosyne` and `$cargo test -p mnemosyne-parser`, respectively.

Finally, `$ cargo doc` will generate the RustDoc API documentation for Mnemosyne and its dependencies. The RustDoc HTML will be output to the `target/doc` directory. The file `target/doc/mnemosyne/index.html` is a natural starting point for the documentation for the entire project.

### Contributing

Since Mnemosyne is currently being implemented as part of my senior thesis research, I cannot accept code contributions from the community at this time. Community feedback and comments, however, are always quite welcome; and if you find any problems or bugs, please do not hesitate report them on the issue tracker. For more information on how to contribute to Mnemosyne, please see [CONTRIBUTING.md](https://github.com/hawkw/mnemosyne/blob/master/CONTRIBUTING.md).
