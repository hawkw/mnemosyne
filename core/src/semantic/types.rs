use super::{ASTNode, SymbolTable, SymbolAnnotation};
use ::position::Position;

use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops;
use std::marker::PhantomData;

// pub type STCell<'a> = Rc<RefCell<SymbolTable<'a>>>;

//==-----------------------------------------------------==
//      Warning: evil typesystem hacking to follow
//==-----------------------------------------------------==
// This is basically a reimplementation of the typestate
// system that Rust briefly had.
macro_rules! scope_typestate_err {
    ($err_site:expr) => {
        panic!("VERY TRAGIC ERROR: Typestate assertion failed during {}.\n \
            A node in the scoped typestate had no scope. Something has gone \
            terribly, terribly wrong. Contact the Mnemosyne implementors.",
            $err_site
        )}
}

pub trait ScopednessTypestate { fn is_scoped() -> bool; }
pub struct Scoped;
pub struct Unscoped;
impl ScopednessTypestate for Scoped { fn is_scoped() -> bool {true} }
impl ScopednessTypestate for Unscoped { fn is_scoped() -> bool {false} }
//==------- exiting typesystem danger zone --------------==

/// An AST node which has been annotated with position &
/// (possibly) scope information.
pub struct Annotated<'a, T, S>
where S: ScopednessTypestate {
    pub node: T,
    pub position: Position,
    scope: Option<SymbolTable<'a>>,
    my_typestate: PhantomData<S>
}

/// Due to Evil Typesystem Hacking reasons, this impl only exists
/// for annotations which are in theScoped typestate.
impl<'a, T> Annotated<'a, T, Scoped>  {

    /// Get the type signature associated with the given name.
    ///
    /// This returns a borrowed reference to the type signature
    /// associated with that name in the current scope. The argument
    /// can be any type `Q` such that `String: Borrow<Q>` (i.e.
    /// you can pass an `&str` to this function).
    pub fn get_type<Q: ?Sized>(&self, name: &'a Q) -> Option<&SymbolAnnotation>
    where String: Borrow<Q>,
          Q: Hash + Eq {
        match self.scope {
            Some(ref table) => table.get(name),
            None => scope_typestate_err!("get_type()")
        }
    }

    /// Check if the given name is defined in this scope.
    ///
    /// The argument can be any type `Q` such that `String: Borrow<Q>`
    /// (i.e. you can pass an `&str` to this function).
    pub fn is_defined_here<Q: ?Sized>(&self, name: &Q) -> bool
    where String: Borrow<Q>,
          Q: Hash + Eq {
        match self.scope {
            Some(ref table) => table.contains_key(name),
            None => scope_typestate_err!("is_defined_here()")
        }
    }

}

impl<'a, T> Annotated<'a, T, Unscoped> {

    /// Consume this unscoped annotation to produce a new
    /// annotation in the scoped typestate with the given
    /// scope.
    pub fn with_scope(self, scope: SymbolTable<'a>)
        -> Annotated<'a, T, Scoped> {
        Annotated {
            node: self.node,
            position: self.position,
            scope: Some(scope),
            my_typestate: PhantomData
        }
    }
}

impl<'a, T, S> ops::Deref for Annotated<'a, T, S>
where S: ScopednessTypestate,
      T: ASTNode {
    type Target = T;
    fn deref(&self) -> &T { &self.node }
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
                   , Double
                   // TODO: finish
                   }
