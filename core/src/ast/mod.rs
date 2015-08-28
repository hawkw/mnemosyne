use std::{fmt, iter};
use super::position::Positional;
use std::rc::Rc;

macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t').take($to)
                                      .collect::<String>() )
}

pub trait ASTNode {

    /// Pretty-print the AST node as an S-expression
    /// at the desired indentation level.
    fn to_sexpr(&self, level: usize) -> String;

}

#[derive(PartialEq, Clone, Debug)]
pub enum Form {
    Define(DefForm),
    // If(IfNode),
    // Let(LetNode),
    // Call(CallNode)
}

impl ASTNode for Form {
    fn to_sexpr(&self, level: usize) -> String {
        unimplemented!()
    }
}

pub type Ident = Positional<String>;
pub type Expr = Box<Positional<Form>>;

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm {
    TopLevel { name: Ident
             , value: Expr
             , annot: Ident
             },
    Function { name: Ident
             , formals: Vec<Positional<Formal>>
             , annot: Ident
             , body: Expr
             }
}

impl ASTNode for DefForm {
    fn to_sexpr(&self, level: usize) -> String {
        let tab = indent!(level);
        match self {
            _ => unimplemented!() // todo: finish
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { name: Ident, annot: Ident }
