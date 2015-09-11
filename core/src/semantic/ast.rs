use ::position::Positional;
use super::annotations::{ Annotated
                        , ScopednessTypestate
                        , ScopedState
                        , Scoped
                        };
use super::types::Type;

use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::Hash;

pub type Ident = Positional<String>;

pub type Expr<'a, S: ScopednessTypestate>
    = Annotated<'a
               , Form<'a, S>
               , S>;

pub type Body<'a, S: ScopednessTypestate> = Vec<Expr<'a, S>>;

pub type Bindings<'a, S: ScopednessTypestate>
    = Vec<Annotated<'a
                   , Binding<'a, S>
                   , S>>;

#[derive(PartialEq, Clone, Debug)]
pub struct Module<'a, S: ScopednessTypestate> {
    pub name: Ident
  , pub exporting: Vec<Ident>
  , pub body: Body<'a, S>
}

impl<'a, S> Module<'a, S> where S: ScopednessTypestate {

    /// Returns true if the namespace is exporting any names
    #[inline] pub fn is_lib (&self) -> bool { !self.exporting.is_empty() }

}

impl<'a> Scoped<'a, Module<'a, ScopedState>>{

    pub fn has_name<Q: ?Sized>(&self, name: &Q) -> bool
    where String: Borrow<Q>
        , String: PartialEq<Q>
        , Q: Hash + Eq
    {
        self.is_defined_here(name) &&
        self.node.exporting
            .iter()
            .find(|ref i| &i.value == name)
            .is_some()
    }

}

#[derive(PartialEq, Clone, Debug)]
pub enum Form<'a, S: ScopednessTypestate> {
    Define(DefForm<'a, S>)
  , If { condition: Rc<Expr<'a, S>>
       , if_clause: Rc<Expr<'a, S>>
       , else_clause: Option<Rc<Expr<'a, S>>>
       }
  , Let(LetForm<'a, S>)
  , Call { fun: Ident
         , body: Body<'a, S>
         }
  , Lambda(Function<'a, S>)
  , Logical(Logical<'a, S>)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident
                  , pub annot: Ident
                  }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm<'a, S: ScopednessTypestate> {
    TopLevel { name: Ident
             , annot: Ident
             , value: Rc<Expr<'a, S>>
             }
  , Function { name: Ident
             , fun: Function<'a,S>
             }
}

/// Logical `and` and `or` expressions
///
/// The general expectation is that these will generally be parsed as infix.
#[derive(PartialEq, Clone, Debug)]
pub enum Logical<'a, S: ScopednessTypestate> {
    And { a: Rc<Expr<'a, S>>
        , b: Rc<Expr<'a, S>>
        }
  , Or { a: Rc<Expr<'a, S>>
       , b: Rc<Expr<'a, S>>
       }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LetForm<'a, S: ScopednessTypestate> {
    Let { bindings: Bindings<'a, S>
        , body: Body<'a, S>
        }
  , Invocation { proc_id: Ident
               , init: Binding<'a, S>
               , body: Body<'a, S>
               }
  , LetRec { bindings: Bindings<'a, S>
           , body: Body<'a, S>
           }
  , LetSplat { bindings: Bindings<'a, S>
             , body: Body<'a, S>
             }

}

#[derive(PartialEq, Clone, Debug)]
pub struct Binding<'a, S: ScopednessTypestate> {
    pub name: Ident
  , pub typ: Type
  , pub value: Rc<Expr<'a, S>>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function<'a, S: ScopednessTypestate> {
    pub formals: Vec<Annotated<'a, Formal, S>>
  , pub annot: Ident
  , pub body: Body<'a, S>
}


#[derive(PartialEq, Clone, Debug)]
pub struct Data<'a, S: ScopednessTypestate> {
    pub name: Ident
  , pub variants: Vec<Variant<'a, S>>
}

impl<'a, S> Data<'a, S>
where S: ScopednessTypestate {
    pub fn is_algebraic(&self) -> bool { self.variants.len() > 1 }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Variant<'a, S: ScopednessTypestate> {
    pub name: Ident
  , pub fields: Vec<Annotated<'a, Formal, S>>
}
