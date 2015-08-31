use super::{ASTNode, SymbolTable};

use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::Hash;

// pub type STCell<'a> = Rc<RefCell<SymbolTable<'a>>>;

pub struct Annotated<'a, T> where T: ASTNode {
    node: T, scope: Rc<SymbolTable<'a>>
}

impl<'a, T> Annotated<'a, T> where T: ASTNode {

    /// Get the type signature associated with the given name.
    ///
    /// This returns a borrowed reference to the type signature
    /// associated with that name in the current scope. The argument
    /// can be any type `Q` such that `String: Borrow<Q>` (i.e.
    /// you can pass an `&str` to this function).
    pub fn get_type<Q: ?Sized>(&self, name: &Q) -> Option<&Type>
    where String: Borrow<Q>, Q: Hash + Eq {
        self.scope.get(name)
    }

    /// Check if the given name is defined in this scope.
    ///
    /// The argument can be any type `Q` such that `String: Borrow<Q>`
    /// (i.e. you can pass an `&str` to this function).
    pub fn is_defined_here<Q: ?Sized>(&self, name: &Q) -> bool
    where String: Borrow<Q>, Q: Hash + Eq {
        self.scope.contains_key(name)
    }

}

#[derive(Debug, Clone)]
pub enum Type { Ref(Reference)
              , Prim(Primitive)
              , Algebraic //TODO: make me a thing
              , Function { params: Vec<Type>, rt: Box<Type> }
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

#[derive(Debug, Clone)]
pub enum Primitive { Int
                   , Uint
                   , Byte
                   , Char
                   , Str
                   , Bool
                   , Symbol(String)
                   // TODO: finish
                   }
