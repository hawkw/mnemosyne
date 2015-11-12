//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Compile
//!
//! This module contains code for compiling Mnemosyne ASTs into LLVM IR.

use std::ffi::CString;
use std::cmp::Ordering;
use std::mem;

use libc::c_uint;

use rustc::lib::llvm;
use rustc::lib::llvm::{ ContextRef
                      , ModuleRef
                      , ValueRef
                      , BuilderRef
                      , TypeRef
                      };

use errors::ExpectICE;
use forktable::ForkTable;
use position::Positional;
use ast::{ Node
         , Form
         , DefForm
         , Ident
         , Function };

use semantic::annotations::{ ScopedState
                           , Scoped
                           };
use semantic::types::*;

/// Result type for compiling an AST node to LLVM IR
///
/// An `IRResult` contains either a `ValueRef`, if compilation was successful,
/// or a `Positional<String>` containing an error message and the position of
/// the line of code which could not be compiled.
pub type IRResult = Result<ValueRef, Vec<Positional<String>>>;

/// Result type for compiling a type to an LLVM `TypeRef`.
pub type TypeResult = Result<TypeRef, Positional<String>>;

pub type NamedValues<'a> = ForkTable<'a, &'a str, ValueRef>;

#[inline] fn word_size() -> usize { mem::size_of::<isize>() }

/// Trait for that which may join in The Great Work
pub trait Compile {
    /// Compile `self` to an LLVM `ValueRef`
    ///
    /// # Returns:
    ///   - `Ok` containing a `ValueRef` if this was compiled correctly.
    ///   - An `Err` with a vector of error messages containing any
    ///     errors that occured during compilation.
    ///
    /// # Panics:
    ///   - If something has gone horribly wrong. This does NOT panic if the
    ///     code could not be compiled because it was incorrect, but it will
    ///     panic in the event of an internal compiler error.
    fn to_ir(&self, context: LLVMContext) -> IRResult;
}

/// Trait for type tags that can be translated to LLVM
pub trait TranslateType {
    /// Translate `self` to an LLVM `TypeRef`
    ///
    /// # Returns:
    ///   - `Ok` containing a `TypeRef` if this was compiled correctly.
    ///   - An `Err` with a positional error message in the event of
    ///     a type error.
    ///
    /// # Panics:
    ///   - In the event of an internal compiler error (i.e. if a well-formed
    ///     type could not be gotten from LLVM correctly).
    fn translate_type(&self, context: LLVMContext) -> TypeResult;
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
                           , pub names: NamedValues<'a>
                           }

/// because we are in the Raw Pointer Sadness Zone (read: unsafe),
/// it is necessary that we assert that everything exists.
macro_rules! not_null {
    ($target:expr) => ({
        let e = $target;
        if e.is_null() {
            ice!( "assertion failed: {} returned null!"
                , stringify!($target)
                );
        } else { e }
    })
}

/// converts a raw pointer that may be null to an Option
/// the compiler will yell about this, claiming that it involves
/// an unused unsafe block, but the unsafe block is usually necessary.
macro_rules! optionalise {
    ($target:expr) => ({
            let e = unsafe { $target };
            if e.is_null() {
                None
            } else { Some(e) }
    })
}

macro_rules! try_vec {
    ($expr:expr) => ({
        if !$expr.is_empty() {
            return Err($expr)
        }
    })
}

impl<'a> LLVMContext<'a> {

    /// Constructs a new LLVM context.
    ///
    /// # Returns:
    ///   - An `LLVMContext`
    ///
    /// # Panics:
    ///   - If the LLVM C ABI returned a null value for the `Context`,
    ///     `Builder`, or `Module`
    pub fn new(module_name: &str) -> Self {
        let name =
            CString::new(module_name)
                .expect_ice(
                    &format!( "Could not create C string for module name: {:?}"
                            , module_name));
        unsafe {
            let ctx = not_null!(llvm::LLVMContextCreate());
            LLVMContext {
                llctx: ctx
              , llmod:  not_null!(llvm::LLVMModuleCreateWithNameInContext(name.into_raw(), ctx))
              , llbuilder: not_null!(llvm::LLVMCreateBuilderInContext(ctx))
              , names: NamedValues::new()
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

    pub fn int_type(&self, size: usize) -> Option<TypeRef> {
        optionalise!(llvm::LLVMIntTypeInContext(self.llctx, size as c_uint))
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

    /// Get any existing declarations for a given function name.
    ///
    /// # Returns:
    ///   - `Some` if there is an existing previous declaration
    ///     for this function.
    ///   - `None` if the function has not been declared previously.
    ///
    /// # Panics:
    ///   - If the C string representation for the function name could
    ///     not be created.
    pub fn existing_decl(&self, name: &Ident) -> Option<ValueRef> {
        CString::new((&name.value).clone())
            .map(|s| optionalise!(s.as_ptr()))
            .map(|o| o.and_then(|p|
                        optionalise!(llvm::LLVMGetNamedFunction(self.llmod, p)))
                )
            .expect_ice(&format!(
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
          , Form::Lit(ref c) => unimplemented!()
          , Form::NameRef(ref form) => unimplemented!()
        }
    }
}

impl<'a> Compile for Scoped<'a, DefForm<'a, ScopedState>> {
    fn to_ir(&self, context: LLVMContext) -> IRResult {
        match **self {
            DefForm::TopLevel { ref name, ref value, .. } =>
                unimplemented!()
         ,  DefForm::Function { ref name, ref fun } => {
                match context.existing_decl(name) {
                    Some(previous) => unimplemented!()
                  , None => unimplemented!()
                }
            }
        }
    }
}


impl<'a> Compile for Scoped<'a, Function<'a, ScopedState>> {

    fn to_ir(&self, context: LLVMContext) -> IRResult {
        let mut errs: Vec<Positional<String>> = vec![];
        // Check to see if the pattern binds an equivalent number of arguments
        // as the function signature (minus one, which is the return type).
        for e in &self.equations {
            match e.pattern_length()
                   .cmp(&self.arity()) {
                // the equation's pattern is shorter than the function's arity
                // eventually, we'll autocurry this, but for now, we error.
                // TODO: maybe there should be a warning as well?
                Ordering::Less => errs.push(Positional {
                    pos: e.position.clone()
                  , value: format!( "[error] equation had fewer bindings \
                                     than function arity\n \
                                     [error] auto-currying is not currently \
                                     implemented.\n \
                                     signature: {}\nfunction: {}\n"
                                  , self.sig
                                  , (*e).to_sexpr(0)
                                  )
                  })
                // the equation's pattern is longer than the function's arity
                // this is super wrong and always an error.
              , Ordering::Greater => errs.push(Positional {
                  pos: e.position.clone()
                , value: format!( "[error] equation bound too many arguments\n \
                                   signature: {}\nfunction: {}\n"
                                , self.sig
                                , (*e).to_sexpr(0)
                                )
              })
            , _ =>  {}
            }
        }
        // TODO: this could be made way more idiomatic...
        try_vec!(errs);

        // Get the function's parameter types
        let mut param_types = self.sig.param_types().iter()
                                  .map(|ty| ty.translate_type(context));
        unimplemented!()
    }

}

impl TranslateType for Type {
    fn translate_type(&self, context: LLVMContext) -> TypeResult {
        match *self {
            Type::Ref(ref reference)  => reference.translate_type(context)
          , Type::Prim(ref primitive) => primitive.translate_type(context)
          , _ => unimplemented!() // TODO: figure this out
        }
    }
}

impl TranslateType for Reference {
    fn translate_type(&self, context: LLVMContext) -> TypeResult {
        unimplemented!() // TODO: figure this out
    }
}

impl TranslateType for Primitive {
    fn translate_type(&self, context: LLVMContext) -> TypeResult {
        Ok(match *self { Primitive::IntSize => context.int_type(word_size())
                    , Primitive::UintSize   => context.int_type(word_size())
                    , Primitive::Int(bits)  => context.int_type(bits as usize)
                    , Primitive::Uint(bits) => context.int_type(bits as usize)
                    , Primitive::Float      => context.float_type()
                    , Primitive::Double     => context.double_type()
                    , Primitive::Byte       => context.byte_type()
                    , _ => unimplemented!() // TODO: figure this out
                    }
            .expect_ice( &format!( "Could not get {:?} type from LLVM"
                                 , *self)
                       )
            )
    }
}
