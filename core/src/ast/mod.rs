use std::{fmt, iter};
use std::rc::Rc;
use std::fmt::Write;

use super::position::Positional;

macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t')
                        .take($to)
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
        match self {
            &Form::Define(ref form) => form.to_sexpr(level),
        }
    }
}

pub type Ident = Positional<String>;
pub type Expr = Box<Positional<Form>>;

impl ASTNode for Ident {
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm {
    TopLevel { name: Ident
             , annot: Ident
             , value: Expr
             },
    Function { name: Ident
             , annot: Ident
             , formals: Vec<Positional<Formal>>
             , body: Expr
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
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { name: Ident, annot: Ident }

impl ASTNode for Formal {
    fn to_sexpr(&self, level: usize) -> String {
        format!("{}: {}", *(self.name), *(self.annot))
    }
}
