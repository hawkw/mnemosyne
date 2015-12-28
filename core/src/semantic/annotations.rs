//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops;
use std::fmt;
use std::marker::PhantomData;

use ast;

use super::{ SymbolAnnotation
           , SymbolTable
           };
use position::{ Position
              , Positional
              };
//==-----------------------------------------------------==
//      Warning: evil typesystem hacking to follow
//==-----------------------------------------------------==
// This is basically a reimplementation of the typestate
// system that Rust briefly had.
macro_rules! scope_typestate_err {
    ($err_site:expr) => {
        ice!("VERY TRAGIC ERROR! Typestate assertion failed: \
            A node in the scoped typestate had no scope. \n \
            During evaluation of {}",
            $err_site)
        }
}
pub trait ScopednessTypestate { fn is_scoped() -> bool; }
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScopedState;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnscopedState;
impl ScopednessTypestate for ScopedState { fn is_scoped() -> bool { true } }
impl ScopednessTypestate for UnscopedState { fn is_scoped() -> bool { false } }
impl fmt::Display for ScopedState {
    fn fmt(&self, f: &mut fmt::Formatter)
        -> fmt::Result { write!(f, "Scoped") }
}
impl fmt::Display for UnscopedState {
    fn fmt(&self, f: &mut fmt::Formatter)
        -> fmt::Result { write!(f,"Unscoped") }
}
//==------- exiting typesystem danger zone --------------==

/// An AST node which has been annotated with position &
/// (possibly) scope information.
#[derive(Clone, Debug)]
pub struct Annotated<'a, T, S>
where S: ScopednessTypestate { pub node: T
                             , pub position: Position
                             , scope: Option<SymbolTable<'a>>
                             , my_typestate: PhantomData<S>
                             }

pub type Scoped<'a, T> = Annotated<'a, T, ScopedState>;
pub type Unscoped<'a, T> = Annotated<'a, T, UnscopedState>;

/// Due to Evil Typesystem Hacking reasons, this impl only exists
/// for annotations which are in the Scoped typestate.
impl<'a, T> Scoped<'a, T> {

    /// Extract the symbol table from this node's scope annotation.
    ///
    /// This fails with the typestate error if there is no symbol table
    /// for this annotation. Of course, since this node is in the scoped state,
    /// this should only ever fail if things have gone horribly wrong.
    pub fn symbol_table(&'a self) -> &'a SymbolTable<'a> {
        match self.scope { Some(ref table) => table
                         , None => scope_typestate_err!("symbol_table()")
                         }
    }

    /// Get the type signature associated with the given name.
    ///
    /// This returns a borrowed reference to the type signature
    /// associated with that name in the current scope. The argument
    /// can be any type `Q` such that `String: Borrow<Q>` (i.e.
    /// you can pass an `&str` to this function).
    pub fn get_type<Q: ?Sized>(&'a self, name: &'a Q)
                               -> Option<&'a SymbolAnnotation>
    where String: Borrow<Q>
             , Q: Hash + Eq {
        match self.scope { Some(ref table) => table.get(name)
                         , None => scope_typestate_err!("get_type()")
                         }
    }

    /// Check if the given name is defined in this scope.
    ///
    /// The argument can be any type `Q` such that `String: Borrow<Q>`
    /// (i.e. you can pass an `&str` to this function).
    pub fn is_defined_here<Q: ?Sized>(&self, name: &Q) -> bool
    where String: Borrow<Q>
             , Q: Hash + Eq {
        match self.scope {
            Some(ref table) => table.contains_key(name),
            None => scope_typestate_err!("is_defined_here()")
        }
    }

}

impl<'a, T> Unscoped<'a, T>
where T: ast::Node {

    /// Consume this unscoped annotation to produce a new
    /// annotation in the scoped typestate with the given
    /// scope.
    pub fn with_scope(self, scope: SymbolTable<'a>) -> Scoped<'a, T>{
        Annotated { node: self.node
                  , position: self.position
                  , scope: Some(scope)
                  , my_typestate: PhantomData
                  }
    }

    pub fn new(node: T, position: Position) -> Self {
        Annotated { node: node
                  , position: position
                  , scope: None
                  , my_typestate: PhantomData
                  }
    }

}

impl<'a, T, S> ops::Deref for Annotated<'a, T, S>
where S: ScopednessTypestate
    , T: ast::Node {
    type Target = T;
    fn deref(&self) -> &T { &self.node }
}

impl<'a, T, S> fmt::Display for Annotated<'a, T, S>
where S: ScopednessTypestate
    , T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.node, self.position)
    }
}

impl<'a, T> From<Positional<T>> for Unscoped<'a, T>
where T: ast::Node {

    fn from(p: Positional<T>) -> Self { Unscoped::new(p.value, p.pos) }
}

// impl<'a, T> fmt::Debug for Annotated<'a, T, Scoped>
// where T: fmt::Debug {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}, annotated with scope", self.node)
//     }
// }
//
// impl<'a, T> fmt::Debug for Annotated<'a, T, Unscoped>
// where T: fmt::Debug {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}, unscoped", self.node)
//     }
// }

impl<'a, T, S> PartialEq for Annotated<'a, T, S>
where S: ScopednessTypestate
    , T: PartialEq {
    fn eq(&self, other: &Annotated<'a, T, S>) -> bool {
        self.node == other.node
    }
}

impl<'a, A, S> Annotated<'a, A, S>
where S: ScopednessTypestate {
    #[inline]
    pub fn map_pos<B>(&self, val: B) -> Positional<B> {
        Positional::from(self.position, val)
    }
}
