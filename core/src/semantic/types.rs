use super::{ ASTNode
           , SymbolTable
           , SymbolAnnotation
           };
use ::position::Position;

use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Primitive { Int
                   , Uint
                   , Byte
                   , Char
                   , Str
                   , Bool
                   , Float
                   , Double
                   // TODO: finish
                   }
