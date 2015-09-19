use ::position::Positional;
use super::annotations::{ Annotated
                        , ScopednessTypestate
                        , ScopedState
                        , Scoped
                        };
use super::types;

use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::Hash;
use std::fmt;

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

    /// Returns true if the namespace is exporting any nages
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
    // TODO: maybe definitions aren't expressions?
    // (they don't return a value...)
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
  , Constant(Const)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident
                  , pub annot: Ident
                  }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm<'a, S: ScopednessTypestate> {
    TopLevel { name: Ident
             , annot: types::Type
             , value: Rc<Expr<'a, S>>
             }
  , Function { name: Ident
             , fun: Function<'a,S>
             }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Class<'a, S: ScopednessTypestate> {
    name: Ident
  , ty_param: Ident
  , defs: Vec<Prototype<'a, S>>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Instance<'a, S: ScopednessTypestate> {
    pub class: Ident
  , pub ty: types::Type
  , pub functions: Vec<Function<'a, S>>
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
pub enum Const { IntConst(i64)
               , UintConst(u64)
               }

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self { Const::IntConst(ref n)    => write!(f, "{}", n)
                    , Const::UintConst(ref n)   => write!(f, "{}", n)
                    }
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
  , pub typ: types::Type
  , pub value: Rc<Expr<'a, S>>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function<'a, S: ScopednessTypestate> {
    pub sig: types::Signature
  , pub body: Body<'a, S>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype<'a, S: ScopednessTypestate> {
    pub formals: Vec<Annotated<'a, Formal, S>>
  , pub annot: Ident
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
