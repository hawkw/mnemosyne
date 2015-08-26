#![crate_name = "mnemosyne"]
#![crate_type = "lib"]
#![feature(rustc_private)]

extern crate rustc;
extern crate combine;

pub mod position;
pub mod ast;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , BuilderRef
                      , ModuleRef
                      , ValueRef
                  };

use position::Positional;

pub type IRResult = Result<ValueRef, Positional<String>>;

/// LLVM compilation context.
///
/// This is based rather loosely on MIT License code from
/// the [iron-kaleidoscope](https://github.com/jauhien/iron-kaleidoscope)
/// tutorial, and from [`librustc_trans`](https://github.com/rust-lang/rust/blob/master/src/librustc_trans/trans/mod.rs)
/// from the Rust compiler.
pub struct LLVMContext {
    pub llctx: ContextRef,
    pub llmod: ModuleRef
}

impl LLVMContext {
    pub fn new(module_name: &str) -> Self {
        unsafe {
            let name = module_name.to_c_str().as_ptr();
            let c = llvm::LLVMContextCreate();
            let m = llvm::LLVMModuleCreateWithNameInContext(c, name);
            LLVMContext { llctx: c, llmod: m }
        }
    }
}

impl Drop for LLVMContext {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeModule(self.llmod);
            llvm::LLVMContextDispose(self.llctx);
        }
    }
}
