use std::ffi::CString;
use std::convert::Into;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                      , BuilderRef
                      , TypeRef
                  };
use seax::compiler_tools::ForkTable;

use position::Positional;
use ast::{Form, DefForm};
use semantic::types::{Type, Reference, Primitive};

pub type IRResult = Result<ValueRef, Positional<String>>;
pub type SymbolTable<'a> = ForkTable<'a, &'a str, ValueRef>;

const WORD_SIZE: c_uint = std::mem::size_of::<isize>();

/// Trait for that which may join in The Great Work
pub trait Compile {
    fn to_ir(&self, context: LLVMContext) -> IRResult;
}

pub trait TranslateType {
    fn translate_type(&self, context: LLVMContext) -> TypeRef;
}

/// LLVM compilation context.
///
/// This is based rather loosely on MIT License code from
/// the [iron-kaleidoscope](https://github.com/jauhien/iron-kaleidoscope)
/// tutorial, and from [`librustc_trans`](https://github.com/rust-lang/rust/blob/master/src/librustc_trans/trans/mod.rs)
/// from the Rust compiler.
pub struct LLVMContext<'a> { pub llctx: ContextRef
                           , pub llmod: ModuleRef
                           , pub llbuilder: BuilderRef
                           , pub names: SymbolTable<'a>
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

impl TranslateType for Type {
    fn translate_type(&self, context: LLVMContext) -> TypeRef {
        match *self {
            Type::Ref(ref reference)  => reference.TranslateType(context),
            Type::Prim(ref primitive) => primitive.translate_type(context),
            _ => unimplemented!() // TODO: figure this out
        }
    }
}

impl TranslateType for Reference {
    fn translate_type(&self, context: LLVMContext) -> TypeRef {
        unimplemented!() // TODO: figure this out
    }
}

impl TranslateType for Primitive {
    fn translate_type(&self, context: LLVMContext) -> TypeRef {
        match *self {
            Primitive::Int => // Integers are machine word size
                llvm::LLVMIntTypeInContext(context, WORD_SIZE),
            Primitive::Float => // Floats are single precision
                llvm::LLVMGetFloatTypeInContext(context),
            Primitive::Double => // Doubles are obvious precision
                llvm::LLVMGetDoubleTypeInContext(context),
            Primitive::Byte => // Bytes are 8 bits (duh)
                llvm::LLVMGetInt8TypeInContext(context)
            _ => unimplemented!() // TODO: figure this out
        }
    }
}
