use std::{fmt, iter};
use std::rc::Rc;
use std::fmt::Write;

use seax::compiler_tools::ForkTable;

pub mod ast;
pub mod types;

use ast::*;
use self::types::{Annotated, Type};

pub type SymbolTable<'a> = ForkTable<'a, String, Type>;

macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t')
                        .take($to)
                        .collect::<String>() )
}

pub trait ASTNode: Sized {

    /// Pretty-print the AST node as an S-expression
    /// at the desired indentation level.
    fn to_sexpr(&self, level: usize) -> String;
    /// Analyse this node & annotate it with the appropriate type state.
    fn analyze<'a>(self, scope: SymbolTable) -> Annotated<'a, Self>;

}

impl ASTNode for Form {

    fn to_sexpr(&self, level: usize) -> String {
        match self {
            &Form::Define(ref form) => form.to_sexpr(level),
        }
    }

    fn analyze<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}

impl ASTNode for Ident {

    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

    fn analyze<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
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

    fn analyze<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}

impl ASTNode for Formal {

    fn to_sexpr(&self, level: usize) -> String {
        format!("{}: {}", *(self.name), *(self.annot))
    }


    fn analyze<'a>(self, scope: SymbolTable) -> Annotated<'a, Self> {
        unimplemented!() //TODO: implement
    }

}
