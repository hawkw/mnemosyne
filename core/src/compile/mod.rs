//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//

use std::ffi::CString;
use std::cmp::Ordering;
use std::mem;

use libc::c_uint;
use llvm_sys::prelude::LLVMValueRef;

use iron_llvm::core;
use iron_llvm::core::types::{ Type
                            , TypeCtor
                            , RealTypeCtor
                            , RealTypeRef
                            , IntTypeCtor
                            , IntTypeRef
                            };
use iron_llvm::{LLVMRef, LLVMRefCtor};

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
use semantic::types;
use semantic::types::{ Primitive
                    , Reference
                    };

/// Result type for compiling an AST node to LLVM IR
///
/// An `IRResult` contains either a `ValueRef`, if compilation was successful,
/// or a `Positional<String>` containing an error message and the position of
/// the line of code which could not be compiled.
pub type IRResult = Result<LLVMValueRef, Vec<Positional<String>>>;

/// Result type for compiling a type to an LLVM `TypeRef`.
pub type TypeResult<T: Type + Sized> = Result<T, Positional<String>>;

pub type NamedValues<'a> = ForkTable<'a, &'a str, LLVMValueRef>;

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
// pub trait TranslateType {
//     /// Translate `self` to an LLVM `TypeRef`
//     ///
//     /// # Returns:
//     ///   - `Ok` containing a `TypeRef` if this was compiled correctly.
//     ///   - An `Err` with a positional error message in the event of
//     ///     a type error.
//     ///
//     /// # Panics:
//     ///   - In the event of an internal compiler error (i.e. if a well-formed
//     ///     type could not be gotten from LLVM correctly).
//     fn translate_type(&self, context: LLVMContext) -> TypeResult;
// }

/// LLVM compilation context.
///
/// This is based rather loosely on MIT License code from
/// the [iron-kaleidoscope](https://github.com/jauhien/iron-kaleidoscope)
/// tutorial, and from [`librustc_trans`](https://github.com/rust-lang/rust/blob/master/src/librustc_trans/trans/mod.rs)
/// from the Rust compiler.
pub struct LLVMContext<'a> { llctx: core::Context
                           , llmod: core::Module
                           , llbuilder: core::Builder
                           , named_vals: NamedValues<'a>
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

// ------------ SEGFAULT EXISTS SOMEWHERE BELOW THIS LINE --------------------

//
// impl<'a> LLVMContext<'a> {
//
//     /// Constructs a new LLVM context.
//     ///
//     /// # Returns:
//     ///   - An `LLVMContext`
//     ///
//     /// # Panics:
//     ///   - If the LLVM C ABI returned a null value for the `Context`,
//     ///     `Builder`, or `Module`
//     pub fn new(module_name: &str) -> Self {
//         LLVMContext { llctx: core::Context::get_global()
//                     , llmod: core::Module::new(module_name)
//                     , llbuilder: core::Builder::new()
//                     , named_vals: NamedValues::new()
//                     }
//     }
//
//     /// Dump the module's contents to stderr for debugging
//     ///
//     /// Apparently this is the only reasonable way to get a textual
//     /// representation of a `Module` in LLVM
//     pub fn dump(&self) { self.llmod.dump() }
//
//     pub fn int_type(&self, size: usize) -> IntTypeRef {
//         IntTypeRef::get_int_in_context(&self.llctx, size as c_uint)
//     }
//
//     pub fn float_type(&self) -> RealTypeRef {
//         RealTypeRef::get_float_in_context(&self.llctx)
//     }
//     pub fn double_type(&self) -> RealTypeRef {
//         RealTypeRef::get_double_in_context(&self.llctx)
//     }
//     pub fn byte_type(&self) -> IntTypeRef {
//         IntTypeRef::get_int8_in_context(&self.llctx)
//     }
//
//     /// Get any existing declarations for a given function name.
//     ///
//     /// # Returns:
//     ///   - `Some` if there is an existing previous declaration
//     ///     for this function.
//     ///   - `None` if the function has not been declared previously.
//     ///
//     /// # Panics:
//     ///   - If the C string representation for the function name could
//     ///     not be created.
//     pub fn get_fn(&self, name: &Ident) -> Option<core::FunctionRef> {
//         self.llmod.get_function_by_name(name.value.as_ref())
//     }
// }

// impl<'a> Compile for Scoped<'a, Form<'a, ScopedState>> {
//     fn to_ir(&self, context: LLVMContext) -> IRResult {
//         match **self {
//             Form::Define(ref form) => unimplemented!()
//           , Form::Let(ref form) => unimplemented!()
//           , Form::If { .. } => unimplemented!()
//           , Form::Call { .. } => unimplemented!()
//           , Form::Lambda(ref fun) => unimplemented!()
//           , Form::Logical(ref exp) => unimplemented!()
//           , Form::Lit(ref c) => unimplemented!()
//           , Form::NameRef(ref form) => unimplemented!()
//         }
//     }
// }
//
// impl<'a> Compile for Scoped<'a, DefForm<'a, ScopedState>> {
//     fn to_ir(&self, context: LLVMContext) -> IRResult {
//         match **self {
//             DefForm::TopLevel { ref name, ref value, .. } =>
//                 unimplemented!()
//          ,  DefForm::Function { ref name, ref fun } => {
//                 match context.get_fn(name) {
//                     Some(previous) => unimplemented!()
//                   , None => unimplemented!()
//                 }
//             }
//         }
//     }
// }
//
//
// impl<'a> Compile for Scoped<'a, Function<'a, ScopedState>> {
//
//     fn to_ir(&self, context: LLVMContext) -> IRResult {
//         let mut errs: Vec<Positional<String>> = vec![];
//         // Check to see if the pattern binds an equivalent number of arguments
//         // as the function signature (minus one, which is the return type).
//         for e in &self.equations {
//             match e.pattern_length()
//                    .cmp(&self.arity()) {
//                 // the equation's pattern is shorter than the function's arity
//                 // eventually, we'll autocurry this, but for now, we error.
//                 // TODO: maybe there should be a warning as well?
//                 Ordering::Less => errs.push(Positional {
//                     pos: e.position.clone()
//                   , value: format!( "[error] equation had fewer bindings \
//                                      than function arity\n \
//                                      [error] auto-currying is not currently \
//                                      implemented.\n \
//                                      signature: {}\nfunction: {}\n"
//                                   , self.sig
//                                   , (*e).to_sexpr(0)
//                                   )
//                   })
//                 // the equation's pattern is longer than the function's arity
//                 // this is super wrong and always an error.
//               , Ordering::Greater => errs.push(Positional {
//                   pos: e.position.clone()
//                 , value: format!( "[error] equation bound too many arguments\n \
//                                    signature: {}\nfunction: {}\n"
//                                 , self.sig
//                                 , (*e).to_sexpr(0)
//                                 )
//               })
//             , _ =>  {}
//             }
//         }
//         // TODO: this could be made way more idiomatic...
//         try_vec!(errs);
//         unimplemented!()
//     }
//
// }
//
//
//
// // impl TranslateType for types::Type {
// //     fn translate_type(&self, context: LLVMContext) -> TypeResult {
// //         match *self {
// //             types::Type::Ref(ref r) => r.translate_type(context)
// //           , types::Type::Prim(ref p) => p.translate_type(context)
// //           , _ => unimplemented!() // TODO: figure this out
// //         }
// //     }
// // }
//
// // impl TranslateType for Reference {
// //     fn translate_type(&self, context: LLVMContext) -> TypeResult {
// //         unimplemented!() // TODO: figure this out
// //     }
// // }
//
// // impl TranslateType for Primitive {
// //     fn translate_type(&self, context: LLVMContext) -> TypeResult {
// //     //     Ok(match *self {
// //     //         Primitive::IntSize => context.int_type(word_size())
// //     //       , Primitive::UintSize => context.int_type(word_size())
// //     //       , Primitive::Int(bits) => context.int_type(bits as usize)
// //     //       , Primitive::Uint(bits) => context.int_type(bits as usize)
// //     //       , Primitive::Float => context.float_type()
// //     //       , Primitive::Double => context.double_type()
// //     //       , Primitive::Byte => context.byte_type()
// //     //       , _ => unimplemented!() // TODO: figure this out
// //     //   })
// //         unimplemented!()
// //     }
// // }
