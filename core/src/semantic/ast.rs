//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::{ fmt
         , iter
         };
use std::fmt::Write;
use std::rc::Rc;

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
    pub name: Ident
  , pub ty_param: Ident
  , pub defs: Vec<Prototype<'a, S>>
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

pub type Variant<'a, S: ScopednessTypestate>
    = Vec<Annotated<'a, Formal, S>>;

#[derive(PartialEq, Clone, Debug)]
pub struct Data<'a, S: ScopednessTypestate> {
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
where S: ScopednessTypestate {

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
where S: ScopednessTypestate {
   #[allow(unused_variables)]
   fn to_sexpr(&self, level: usize) -> String {
       match *self {
           Form::Define(ref form)  => form.to_sexpr(level)
         , Form::Let(ref form)     => form.to_sexpr(level)
         , Form::If { .. }         => unimplemented!()
         , Form::Call { ref fun, ref body } =>
               format!( "({}{})"
                      , fun.to_sexpr(level)
                      , body.iter()
                            .fold(String::new(), |mut s, expr| {
                                  write!(s, " {}", expr.to_sexpr(level))
                                      .expect("Could not write to string!");
                                  s
                             }))
         , Form::Lambda(ref fun)   => fun.to_sexpr(level)
         , Form::Logical(ref form) => form.to_sexpr(level)
         , Form::Constant(ref c)   => format!("{}", c)
         , Form::NameRef(ref n)    => n.to_sexpr(level)
       }
   }

}

impl<'a, S> Node for DefForm<'a, S>
where S: ScopednessTypestate {

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
where S: ScopednessTypestate {

    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            LetForm::Let { ref bindings, ref body } =>
                format!("{}(let [{}]\n{})"
                    , indent!(level)
                    , bindings.iter()
                              .fold(String::new(), |mut s, binding| {
                                    write!(s, "{}\n", binding.to_sexpr(level))
                                        .expect("Could not write to string!");
                                    s
                                 })
                    , body.iter()
                          .fold(String::new(), |mut s, expr| {
                              write!(s, "{}", expr.to_sexpr(level + 1))
                                .expect("Could not write to string!");
                              s
                          })
                       )
          , LetForm::LetRec { ref bindings, ref body } =>
                format!("{}(letrec [{}]\n{})"
                    , indent!(level)
                    , bindings.iter()
                              .fold(String::new(), |mut s, binding| {
                                    write!(s, "{}\n", binding.to_sexpr(level))
                                        .expect("Could not write to string!");
                                    s
                               })
                    , body.iter()
                          .fold(String::new(), |mut s, expr| {
                                write!(s, "{}", expr.to_sexpr(level + 1)); s
                            })
                          )
          , LetForm::LetSplat { ref bindings, ref body } =>
                format!("{}(let* [{}]\n{})"
                    , indent!(level)
                    , bindings.iter()
                              .fold(String::new(), |mut s, binding| {
                                    write!(s, "{}\n", binding.to_sexpr(level))
                                        .expect("Could not write to string!");
                                    s
                               })
                    , body.iter()
                          .fold(String::new(), |mut s, expr| {
                                write!(s, "{}", expr.to_sexpr(level + 1))
                                    .expect("Could not write to string!");
                                s
                            })
                          )
          , _ => unimplemented!()
        }
    }

}

impl<'a, S> Node for Logical<'a, S>
where S: ScopednessTypestate
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
{
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        unimplemented!()
    }
}

impl<'a, S> Node for Binding<'a, S>
where S: ScopednessTypestate
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
        match *self  { NameRef::Owned(ref name)      => format!("{}", name)
                     , NameRef::Borrowed(ref name)   => format!("&{}", name)
                     , NameRef::Deref(ref name)      => format!("*{}", name)
                     , NameRef::Unique(ref name)     => format!("@{}", name)
                     }

    }
}

impl Node for Ident {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

}
