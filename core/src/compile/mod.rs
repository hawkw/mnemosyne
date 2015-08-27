use std::ffi::CString;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                  };
use seax::compiler_tools::ForkTable;

use position::Positional;

pub type IRResult = Result<ValueRef, Positional<String>>;
pub type SymbolTable<'a> = ForkTable<'a, &'a str, ValueRef>;

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
pub struct LLVMContext<'a> {
    pub llctx: ContextRef,
    pub llmod: ModuleRef,
    pub names: SymbolTable<'a>
}

// because we are in the Raw Pointer Sadness Zone (read: unsafe),
// it is necessary that we assert that everything exists.
macro_rules! not_null {
    ($target:expr) => {
        if $target.is_null() {
            panic!("assertion failed: {} null!", stringify!($target));
        }
    }
}

impl<'a> LLVMContext<'a> {
    pub fn new(module_name: &str) -> Self {
        let name = CString::new(module_name).unwrap();
        unsafe {
            let c = llvm::LLVMContextCreate();
            not_null!(c);
            let m = llvm::LLVMModuleCreateWithNameInContext(name.into_raw(),c);
            not_null!(m);
            LLVMContext { llctx: c, llmod: m, names: SymbolTable::new() }
        }
    }
}

impl<'a> Drop for LLVMContext<'a> {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeModule(self.llmod);
            llvm::LLVMContextDispose(self.llctx);
        }
    }
}
