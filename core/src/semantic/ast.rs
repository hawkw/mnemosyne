//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Mnemosyne abstract syntax tree

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::{ fmt
         , iter
         };
use std::fmt::Write;
use std::rc::Rc;

use itertools::Itertools;

use ::chars;
use ::position::Positional;
use super::annotations::{ Annotated
                        , ScopednessTypestate
                        , ScopedState
                        , Scoped
                        };
use super::types;
use super::SymbolTable;

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

/// Concatenate the S-expression representation of an iterator
/// over `Nodes` into a `String`.
macro_rules! concat_exprs {
    ($it:expr, $level:expr) => {
        $it.iter()
           .map(|expr| expr.to_sexpr($level) )
           .intersperse(String::from(" "))
           .collect::<String>()
        };
    ($it:expr, $level:expr, $sep:expr) => {
        $it.iter()
           .map(|expr| expr.to_sexpr($level) )
           .intersperse(String::from($sep))
           .collect::<String>()
        }
}

/// Trait for a node in the abstract syntax tree.
///
/// This provides a visitor method for semantic analysis (which may be split
/// into multiple transforms later), and a method for formatting the AST node
/// as an S-expression.
pub trait Node {

   /// Pretty-print the AST node as an S-expression at the desired
   /// indentation level.]
   ///
   /// Note that this prints the desugared form and may not be a perfect
   /// match for the original source code.
   fn to_sexpr(&self, level: usize) -> String;

}

/// To format a node for display, just use the S-expression representation.
impl fmt::Display for Node + Sized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_sexpr(0))
    }
}

pub trait AnnotateTypes<'a>: Sized {

   #[allow(unused_variables)]
   fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self> {
       unimplemented!() //TODO: implement
   }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Module<'a, S: ScopednessTypestate>
where S: 'a {
    pub name: Ident
  , pub exporting: Vec<Ident>
  , pub body: Body<'a, S>
}

impl<'a, S> Module<'a, S>
where S: ScopednessTypestate
    , S: 'a {

    /// Returns true if the namespace is exporting any names
    #[inline] pub fn is_lib (&self) -> bool { !self.exporting.is_empty() }

}

impl<'a> Scoped<'a, Module<'a, ScopedState>>{

    /// Returns true if the namespace contains a given name.
    pub fn contains_name<Q: ?Sized>(&self, name: &Q) -> bool
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
pub enum Form<'a, S>
where S: ScopednessTypestate
    , S: 'a {

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
  , Lit(Literal)
  , NameRef(NameRef)
}

#[derive(PartialEq, Clone, Debug)]
pub enum NameRef { Owned(Ident)
                 , Borrowed(Ident)
                 , Deref(Ident)
                 , Unique(Ident)
                 }

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident
                  , pub annot: Ident
                  }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    TopLevel { name: Ident
             , annot: types::Type
             , value: Rc<Expr<'a, S>>
             }
  , Function { name: Ident
             , fun: Function<'a,S>
             }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Class<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    pub name: Ident
  , pub ty_param: Ident
  , pub defs: Vec<Prototype<'a, S>>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Instance<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    pub class: Ident
  , pub ty: types::Type
  , pub functions: Vec<Function<'a, S>>
}

/// Logical `and` and `or` expressions
///
/// The general expectation is that these will generally be parsed as infix.
#[derive(PartialEq, Clone, Debug)]
pub enum Logical<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    And { a: Rc<Expr<'a, S>>
        , b: Rc<Expr<'a, S>>
        }
  , Or { a: Rc<Expr<'a, S>>
       , b: Rc<Expr<'a, S>>
       }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal { IntConst(i64)
                 , UintConst(u64)
                 , StringLit(String)
                 }

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self { Literal::IntConst(ref n)    => write!(f, "{}", n)
                    , Literal::UintConst(ref n)   => write!(f, "{}", n)
                    , Literal::StringLit(ref s)   => write!(f, "{}", s)
                    }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LetForm<'a, S>
where S: ScopednessTypestate
    , S: 'a {

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
pub struct Binding<'a, S>
where S: ScopednessTypestate
    , S: 'a { pub name: Ident
            , pub typ: types::Type
            , pub value: Rc<Expr<'a, S>>
            }

#[derive(PartialEq, Clone, Debug)]
pub struct Function<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    pub sig: types::Signature
  , pub equations: Vec<Annotated<'a, Equation<'a, S>, S>>
}

impl<'a, S> Annotated<'a, Function<'a, S>, S>
where S: ScopednessTypestate
    , S: 'a {

    /// Returns the arity of this function's signature
    pub fn arity(&self) -> usize {
        self.node.sig
            .arity()
    }
}

/// A pattern is a vector of pattern elements.
///
/// When entering a function scope that binds using patterns, we should
/// check that the pattern's length matches the arity of the function,
/// until autocurrying is implemented (after the Glorious Revolution).
pub type Pattern = Vec<PatElement>;

impl Node for Pattern {
   #[allow(unused_variables)]
   fn to_sexpr(&self, level: usize) -> String {
       format!( "({})", concat_exprs!(self, level) )
   }

}

/// An element in a pattern-matching expression.
///
/// A pattern element can be a name binding, a typed name binding,
/// a literal, or the magic underscore (anything). Destructuring bind
/// will come later, as will equality.
#[derive(PartialEq, Clone, Debug)]
pub enum PatElement { // eventually this could just be implemented using an Expr,
                      // but it's currently this way because the types of patterns
                      // that you can write are currently quite limited
                      Name(Ident)
                    , Typed { name: Ident, ty: types::Type }
                    , Lit(Literal)
                      // TODO: write literal types for other things (i.e. lists)
                    , Anything
                    }

impl Node for PatElement {
   #[allow(unused_variables)]
   fn to_sexpr(&self, level: usize) -> String {
       match *self {
           PatElement::Name(ref n) => n.to_sexpr(level)
         , PatElement::Typed { ref name, ref ty } =>
            format!( "{}: {}"
                   , name.to_sexpr(level)
                   , ty )
        , PatElement::Lit(ref c) => format!("{}", c)
        , PatElement::Anything => String::from("_")
       }
   }

}

/// A function equation definition
#[derive(PartialEq, Clone, Debug)]
pub struct Equation<'a, S>
where S: ScopednessTypestate
    , S: 'a { pub pattern: Pattern
            , pub body: Body<'a, S>
            }

impl<'a, S> Node for Equation<'a, S>
where S: ScopednessTypestate  {

    fn to_sexpr(&self, level: usize) -> String {
        format!( "({} {})"
               , self.pattern
                     .to_sexpr(level)
               , concat_exprs!( self.body
                              , level + 1
                              , format!("\n{}", indent!(level + 1))
                              )
              )
    }

}

impl<'a, S> Annotated<'a, Equation<'a, S>, S>
where S: ScopednessTypestate
    , S: 'a {

    /// Returns the number of bindings in this equation's pattern
    pub fn pattern_length(&self) -> usize { self.pattern.len() }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype<'a, S: ScopednessTypestate> {
    pub formals: Vec<Annotated<'a, Formal, S>>
  , pub annot: Ident
}

pub type Variant<'a, S: ScopednessTypestate>
    = Vec<Annotated<'a, Formal, S>>;

#[derive(PartialEq, Clone, Debug)]
pub struct Data<'a, S>
where S: ScopednessTypestate
    , S: 'a {
    pub name: Ident
  , pub variants: HashMap<Ident, Variant<'a, S>>
}

// #[derive(PartialEq, Clone, Debug)]
// pub enum Data<'a, S: ScopednessTypestate> {
//     Sum { name: Ident
//         , variants: Vec<Type> }
//   , Product { name: Ident
//             , fields: Vec<Binding<'a, S>> }
// }

impl<'a, S> Data<'a, S>
where S: ScopednessTypestate
    , S: 'a {

    /// Returns true if this ADT is a sum type
    pub fn is_sum_type(&self) -> bool {
        self.variants.len() > 1
    }

    /// Returns true if this ADT is a product type
    #[inline] pub fn is_product_type(&self) -> bool {
        !self.is_sum_type()
    }

    /// Returns the fields of a product type.
    ///
    /// # Returns
    ///  + A `Some<Vec<Annotated<'a, Formal, S>>>` representing
    ///    this type's fields, if this is the definition of a
    ///    product type.
    ///  + Otherwise, `None`
    pub fn get_product_fields(&self) -> Option<&Variant<'a, S>> {
        self.variants.get(&self.name)
    }
}

impl<'a, S> Node for Form<'a, S>
where S: ScopednessTypestate
    , S: 'a {
   #[allow(unused_variables)]
   fn to_sexpr(&self, level: usize) -> String {
       match *self {
           Form::Define(ref form)  => form.to_sexpr(level)
         , Form::Let(ref form)     => form.to_sexpr(level)
         , Form::If { .. }         => unimplemented!()
         , Form::Call { ref fun, ref body } =>
               format!( "({} {})"
                      , fun.to_sexpr(level)
                      , concat_exprs!(body, level) )
         , Form::Lambda(ref fun)   => fun.to_sexpr(level)
         , Form::Logical(ref form) => form.to_sexpr(level)
         , Form::Lit(ref c)   => format!("{}", c)
         , Form::NameRef(ref n)    => n.to_sexpr(level)
       }
   }

}

impl<'a, S> Node for DefForm<'a, S>
where S: ScopednessTypestate
    , S: 'a {

    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            DefForm::TopLevel { ref name, ref annot, ref value } =>
                format!("{}(define {} {} {})"
                  , indent!(level)
                  , name.to_sexpr(level)
                  , annot
                  , value.to_sexpr(level + 1)
                  )
          , DefForm::Function { ref name, ref fun } =>
                format!("{}(define {} {}\n)"
                  , indent!(level)
                  , name.to_sexpr(level)
                  , fun.to_sexpr(level)
                  )
        }
    }

}

impl<'a, S> Node for LetForm<'a, S>
where S: ScopednessTypestate
    , S: 'a {

    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            LetForm::Let { ref bindings, ref body } =>
                format!("{}(let [{}]\n{})"
                    , indent!(level)
                    , concat_exprs!(bindings, level, "\n")
                    , concat_exprs!(body, level + 1)
                    )
          , LetForm::LetRec { ref bindings, ref body } =>
                format!("{}(letrec [{}]\n{})"
                    , indent!(level)
                    , concat_exprs!(bindings, level, "\n")
                    , concat_exprs!(body, level + 1)
                    )
          , LetForm::LetSplat { ref bindings, ref body } =>
                format!("{}(let* [{}]\n{})"
                    , indent!(level)
                    , concat_exprs!(bindings, level, "\n")
                    , concat_exprs!(body, level + 1)
                    )
          , _ => unimplemented!()
        }
    }

}

impl<'a, S> Node for Logical<'a, S>
where S: ScopednessTypestate
    , S: 'a
{
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            Logical::And { ref a, ref b } =>
                format!( "(and {} {})"
                       , a.to_sexpr(level)
                       , b.to_sexpr(level)
                       )
         ,  Logical::Or { ref a, ref b }  =>
                format!( "(and {} {})"
                       , a.to_sexpr(level)
                       , b.to_sexpr(level)
                       )
        }
    }
}

impl<'a, S> Node for Function<'a, S>
where S: ScopednessTypestate
    , S: 'a
{
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        format!( "({} {}\n{}{})"
               , chars::LAMBDA
               , self.sig.to_sexpr(level)
               , indent!(level + 1)
               , concat_exprs!( self.equations
                              , level + 1
                              , format!("\n{}", indent!(level + 1))
                              )
            )
    }
}

impl<'a, S> Node for Binding<'a, S>
where S: ScopednessTypestate
    , S: 'a
{
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        unimplemented!()
    }
}

impl Node for Formal {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        format!("{}: {}", *(self.name), *(self.annot))
    }

}

impl Node for NameRef {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        match *self  { NameRef::Owned(ref name)      => format!("{}", **name)
                     , NameRef::Borrowed(ref name)   => format!("&{}", **name)
                     , NameRef::Deref(ref name)      => format!("*{}", **name)
                     , NameRef::Unique(ref name)     => format!("@{}", **name)
                     }

    }
}

impl Node for Ident {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

}
