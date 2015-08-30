use super::{ASTNode, SymbolTable};

use std::rc::Rc;
use std::cell::RefCell;

pub type STCell<'a> = Rc<RefCell<SymbolTable<'a>>>;

pub struct Annotated<'a, T> where T: ASTNode {
    node: T, scope: STCell<'a>
}

#[derive(Debug, Clone)]
pub enum Type { Ref(Reference)
              , Prim(Primitive)
              , Algebraic //TODO: make me a thing
              }

/// Reference types
///
/// TODO: how will lifetime analysis actually work?
#[derive(Debug, Clone)]
pub enum Reference { Borrowed(Box<Type>) // TODO: borrowed from where?
                   , Moved(Box<Type>) // TODO: moved from where?
                   , Owned(Box<Type>)
                   , Raw(Box<Type>)
                   }

#[derive(Debug, Clone, Copy)]
pub enum Primitive { Int, Uint, Byte, Str // TODO: finish
}
