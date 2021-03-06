//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Mnemosyne semantic analysis
use std::rc::Rc;

use ::forktable::ForkTable;

use ast::*;
use self::annotations::*;
use self::types::Type;

/// A symbol table is a `ForkTable` mapping `String`s to `Type` annotations.
///
/// As of `84f1de8`, symbol annotations can be type annotations for values
/// or annotations for type definitions. This is provisional; type defs
/// may be moved to a separate table later.
///
/// This table should be forked upon entering a new scope.
pub type SymbolTable<'a> = ForkTable<'a, String, SymbolAnnotation<'a>>;

#[macro_use]
macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t')
                        .take($to)
                        .collect::<String>() )
}

pub mod ast;
pub mod types;
pub mod annotations;

impl<'a> AnnotateTypes<'a> for Unscoped<'a, Form<'a, UnscopedState>> {
    #[allow(unused_variables)]
    fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self> {
        unimplemented!() //TODO: implement
    }
}

impl<'a> AnnotateTypes<'a> for Unscoped<'a, DefForm<'a, UnscopedState>> {
    #[allow(unused_variables)]
    fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self>{
        unimplemented!() //TODO: implement
    }
}


#[derive(Clone,Debug,PartialEq)]
pub enum SymbolAnnotation<'a> {
    TypeDef(Type)
  , Value { /// The type of the symbol
            ty: Type,
            /// An optional proven value for the symbol.
            ///
            /// This should be defined iff the symbol signifies a constant value
            /// or a constant expression, or if we were able to prove that the value
            /// remains constant within the current scope.
            proven_value: Option<Rc<Expr<'a, ScopedState>>>
          }
}

impl<'a> SymbolAnnotation<'a> {
    // TODO: possibly this should recurse into references? IDK
    pub fn is_fn_type(&self) -> bool {
        match *self {
            SymbolAnnotation::TypeDef(ref ty) =>
                match *ty { Type::Function(_) => true
                          , _                => false
                          }
          , SymbolAnnotation::Value { ref ty, .. } =>
                match *ty { Type::Function(_) => true
                          , _                => false
                          }
        }
    }

    pub fn is_prim_type(&self) -> bool {
        match *self {
            SymbolAnnotation::TypeDef(ref ty) =>
                match *ty { Type::Prim(_) => true
                          , _             => false
                          }
          , _ => false
        }
    }
}
