use std::ffi::CString;
use std::convert::Into;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                      , BuilderRef
                  };
use seax::compiler_tools::ForkTable;

use position::Positional;
use ast::{Form, DefForm};

pub type IRResult = Result<ValueRef, Positional<String>>;
pub type SymbolTable<'a> = ForkTable<'a, &'a str, ValueRef>;

/// Trait for that which may join in The Great Work
pub trait Compile {
    fn to_ir(&self, context: LLVMContext) -> IRResult;
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
    pub llbuilder: BuilderRef,
    pub names: SymbolTable<'a>
}

// because we are in the Raw Pointer Sadness Zone (read: unsafe),
// it is necessary that we assert that everything exists.
macro_rules! not_null {
    ($target:expr) => ({
        let e = $target;
        if e.is_null() {
            panic!("assertion failed: {} returned null!", stringify!($target));
        } else { e }
    })
}

impl<'a> LLVMContext<'a> {

    pub fn new(module_name: &str) -> Self {
        let name = CString::new(module_name).unwrap();
        unsafe {
            let ctx = not_null!(llvm::LLVMContextCreate());
            LLVMContext {
                llctx: ctx
              , llmod: not_null!(llvm::LLVMModuleCreateWithNameInContext(name.into_raw(), ctx))
              , llbuilder: not_null!(llvm::LLVMCreateBuilderInContext(ctx))
              , names: SymbolTable::new()
            }
        }
    }

    /// Dump the module's contents to stderr for debugging
    ///
    /// Apparently this is the only reasonable way to get a textual
    /// representation of a `ModuleRef` in `librustc_llvm`...
    pub fn dump(&self) {
        unsafe { llvm::LLVMDumpModule(self.llmod); }
    }
}

impl<'a> Drop for LLVMContext<'a> {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeModule(self.llmod);
            llvm::LLVMDisposeBuilder(self.llbuilder);
            llvm::LLVMContextDispose(self.llctx);
        }
    }
}

impl Compile for Form {
    fn to_ir(&self, context: LLVMContext) -> IRResult {
        match self {
            &Form::Define(ref form) => form.to_ir(context)
        }
    }
}

impl Compile for DefForm {
    fn to_ir(&self, context: LLVMContext) -> IRResult {
        match *self {
            DefForm::TopLevel { ref name, ref value, .. } =>
                unimplemented!(),
            DefForm::Function { ref name, ref body, .. } =>
                unsafe {
                    let name_ptr // function name as C string pointer
                        = CString::new((&name.value).clone()).unwrap().as_ptr();
                    let prev_decl // check LLVM module for previous declaration
                        = llvm::LLVMGetNamedFunction(context.llmod, name_ptr);

                    if !prev_decl.is_null() { // a previous declaration exists
                        unimplemented!() // TODO: overloading rules happen here
                    } else { // new function declaration
                        unimplemented!()
                    }
                }
        }
    }
}
