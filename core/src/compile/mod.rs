use std::ffi::CString;
use std::mem;

use libc::c_uint;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                      , BuilderRef
                      , TypeRef
                      };

use forktable::ForkTable;
use position::Positional;
use ast::{ Form
         , DefForm
         , Ident };

use semantic::annotations::{ ScopedState
                           , Scoped
                           };
use semantic::types::*;

pub type IRResult = Result<ValueRef, Positional<String>>;
pub type SymbolTable<'a> = ForkTable<'a, &'a str, ValueRef>;

#[inline] fn word_size() -> c_uint { mem::size_of::<isize>() as c_uint }

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

macro_rules! optionalise {
    ($target:expr) => ({
            let e = unsafe { $target };
            if e.is_null() {
                None
            } else { Some(e) }
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

    pub fn isize_type(&self) -> Option<TypeRef> {
        optionalise!(llvm::LLVMIntTypeInContext(self.llctx, word_size()))
    }

    pub fn float_type(&self) -> Option<TypeRef> {
        optionalise!(llvm::LLVMFloatTypeInContext(self.llctx))
    }
    pub fn double_type(&self) -> Option<TypeRef> {
        optionalise!(llvm::LLVMDoubleTypeInContext(self.llctx))
    }
    pub fn byte_type(&self) -> Option<TypeRef> {
        optionalise!(llvm::LLVMInt8TypeInContext(self.llctx))
    }

    pub fn get_fn_decl(&self, name: &Ident) -> Option<ValueRef> {
        CString::new((&name.value).clone())
            .map(|s| optionalise!(s.as_ptr()))
            .map(|o| o.and_then(|p|
                        optionalise!(llvm::LLVMGetNamedFunction(self.llmod, p)))
                )
            .expect(&format!(
                    "Could not create C string for function name: {:?}"
                    , name
                  ))
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

impl<'a> Compile for Scoped<'a, Form<'a, ScopedState>> {
    fn to_ir(&self, context: LLVMContext) -> IRResult {
        match **self {
            Form::Define(ref form) => unimplemented!()
          , Form::Let(ref form) => unimplemented!()
          , Form::If { .. } => unimplemented!()
          , Form::Call { .. } => unimplemented!()
          , Form::Lambda(ref fun) => unimplemented!()
          , Form::Logical(ref exp) => unimplemented!()
        }
    }
}

impl<'a> Compile for Scoped<'a, DefForm<'a, ScopedState>> {
    fn to_ir(&self, context: LLVMContext) -> IRResult {
        match **self {
            DefForm::TopLevel { ref name, ref value, .. } =>
                unimplemented!()
         ,  DefForm::Function { ref name, ref fun } => {
                match context.get_fn_decl(name) {
                    Some(previous) => unimplemented!()
                  , None => unimplemented!()
                }
            }
        }
    }
}

impl TranslateType for Type {
    fn translate_type(&self, context: LLVMContext) -> TypeRef {
        match *self {
            Type::Ref(ref reference)  => reference.translate_type(context)
          , Type::Prim(ref primitive) => primitive.translate_type(context)
          , _ => unimplemented!() // TODO: figure this out
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
        match *self { Primitive::Int      => context.isize_type()
                    , Primitive::Float    => context.float_type()
                    , Primitive::Double   => context.double_type()
                    , Primitive::Byte     => context.byte_type()
                    , _ => unimplemented!() // TODO: figure this out
                    }.expect(
                        &format!("Could not get {:?} type from LLVM", *self)
                        )
    }
}
