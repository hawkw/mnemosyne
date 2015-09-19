use std::iter;
use std::fmt::Write;
use std::rc::Rc;

use ::forktable::ForkTable;

use ast::*;
use self::annotations::*;
use self::types::Type;

/// A symbol table is a `ForkTable` mapping `String`s to `Type` annotations.
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
pub struct SymbolAnnotation<'a> {
    /// The type of the symbol
    pub ty: Type,
    /// An optional proven value for the symbol.
    ///
    /// This should be defined iff the symbol signifies a constant value
    /// or a constant expression, or if we were able to prove that the value
    /// remains constant within the current scope.
    pub proven_value: Option<Rc<Expr<'a, ScopedState>>>
}
