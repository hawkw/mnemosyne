use std::{fmt, iter};
use std::rc::Rc;
use std::fmt::Write;

use seax::compiler_tools::ForkTable;

pub mod ast;
pub mod types;

use ast::*;
use self::types::{Annotated, Type};

/// A symbol table is a `ForkTable` mapping `String`s to `Type` annotations.
///
/// This table should be forked upon entering a new scope.
pub type SymbolTable<'a> = ForkTable<'a, String, Type>;

macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t')
                        .take($to)
                        .collect::<String>() )
}

/// Trait for a node in the abstract syntax tree.
///
/// This provides a visitor method for semantic analysis (which may be split
/// into multiple transforms later), and a method for formatting the AST node
/// as an S-expression.
pub trait ASTNode: Sized {

    /// Pretty-print the AST node as an S-expression at the desired
    /// indentation level.
    ///
    /// Note that this prints the desugared form and may not be a perfect
    /// match for the original source code.
    fn to_sexpr(&self, level: usize) -> String;

    /// Analyse this node & annotate it with the appropriate type state.
    ///
    /// The `Annotated` type stores a reference to an AST node along with
    /// the definitions visible at that node's scope.
    ///
    /// Essentially, this transforms our early IR (where type information
    /// is just stored in strings from the source program) into a working
    /// representation where types and symbol definitions are encoded in a
    /// way the compiler can analyze.
    fn annotate_types<'a>(self, scope: SymbolTable) -> Annotated<'a, Self>;

}

impl ASTNode for Form {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        match self {
            &Form::Define(ref form) => form.to_sexpr(level),
        }
    }

    #[allow(unused_variables)]
    fn annotate_types<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}

impl ASTNode for Ident {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

    #[allow(unused_variables)]
    fn annotate_types<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}

impl ASTNode for DefForm {

    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            DefForm::TopLevel { ref name, ref annot, ref value } =>
                format!("{}(define {} {} {})", indent!(level),
                    name.to_sexpr(level),
                    annot.to_sexpr(level),
                    value.to_sexpr(level+1)
                    ),
            DefForm::Function { ref name, ref annot, ref formals, ref body } =>
                format!("{}(define {} {} {}\n{})", indent!(level),
                    name.to_sexpr(level),
                    annot.to_sexpr(level),
                    formals.iter().fold(String::new(), |mut s, f| {
                        s.push_str(&f.to_sexpr(level)); s
                        }),
                    body.to_sexpr(level + 1)
                    )
        }
    }

    #[allow(unused_variables)]
    fn annotate_types<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}

impl ASTNode for Formal {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        format!("{}: {}", *(self.name), *(self.annot))
    }

    #[allow(unused_variables)]
    fn annotate_types<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}
