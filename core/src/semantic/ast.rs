use ::position::Positional;
use super::types::{Annotated, ScopednessTypestate};
use std::rc::Rc;

pub type Ident = Positional<String>;
pub type Expr<'a, S: ScopednessTypestate> = Rc<Annotated<'a, Form<'a, S>, S>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Form<'a, S: ScopednessTypestate> {
    Define(DefForm<'a, S>),
    // If(IfNode),
    // Let(LetNode),
    // Call(CallNode)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident, pub annot: Ident }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm<'a, S: ScopednessTypestate> {
    TopLevel { name: Ident
             , annot: Ident
             , value: Expr<'a, S>
             },
    Function { name: Ident
             , annot: Ident
             , formals: Vec<Annotated<'a, Formal, S>>
             , body: Expr<'a, S>
             }
}
