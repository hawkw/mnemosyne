use std::ffi::CString;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                  };

use position::Positional;

pub type IRResult = Result<ValueRef, Positional<String>>;

/// Trait for an object that can be compiled to LLVM IR
pub trait ToIR {
    fn to_ir(&self) -> IRResult;
}

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

// because we are in the Raw Pointer Sadness Zone (read: unsafe),
// it is necessary that we assert that everything exists.
macro_rules! not_null {
    ($target:expr) => {
        if $target.is_null() {
            panic!("assertion failed: {} was null!", stringify!($target));
        }
    }
}

impl LLVMContext {
    pub fn new(module_name: &str) -> Self {
        let name = CString::new(module_name).unwrap();
        unsafe {
            let c = llvm::LLVMContextCreate();
            not_null!(c);
            let m = llvm::LLVMModuleCreateWithNameInContext(name.into_raw(),c);
            not_null!(m);
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
