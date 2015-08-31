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
pub enum Type {
    /// Reference types
    Ref(Reference),
    /// Primitive types
    Prim(Primitive),
    /// An algebraic data type.
    ///
    /// Represented as a vector of variants.
    Algebraic(Vec<Type>),
    /// A function.
    ///
    /// Represented as a vector of parameters and a return type.
    Function { params: Vec<Type>
             , rt: Rc<Type> },
    /// A unique symbol type (`'symbol` syntax)
    Symbol(String)
}

/// Reference types (pointers)
///
/// TODO: how will lifetime analysis actually work?
#[derive(Debug, Clone)]
pub enum Reference {
    /// A reference borrowed from another scope.
    ///
    /// Semantically similar to Rust's `&`-pointers.
    ///
    /// TODO: should this track where it was borrowed from?
    /// (can we even perform this analysis at this stage?)
    Borrowed(Rc<Type>),
    /// A moved reference from another scope
    ///
    /// TODO: should this track where it was moved from?
    /// (can we even perform this analysis at this stage?)
    Moved(Rc<Type>),
    /// A unique (i.e. boxed) reference.
    Unique(Rc<Type>),
    /// A raw (unsafe) reference.
    ///
    /// Unfortunately we have to have this because of
    /// reasons (i.e, FFI).
    Raw(Rc<Type>)
}

/// Language primitive types
///
/// TODO: add some form of provable-refinement (i.e. we know that some value
/// is not just a bool at compile time but that it's true/false, or we know
/// some value is not just a number but the number 1382, or whatever).
#[derive(Debug, Clone, Copy)]
pub enum Primitive { Int
                   , Uint
                   , Byte
                   , Char
                   , Str
                   , Bool
                   , Float
                   // TODO: finish
                   }
