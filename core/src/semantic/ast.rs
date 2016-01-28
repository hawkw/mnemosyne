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
                        , UnscopedState
                        , ScopedState
                        , Scoped
                        };
use super::types;
use super::{SymbolTable, SymbolAnnotation};
use ::{CompileResult, Errors};

pub type Ident = Positional<String>;

pub type Expr<'a, S: ScopednessTypestate>
    = Annotated<'a
               , Form<'a, S>
               , S>;

pub type Body<'a, S: ScopednessTypestate>
    = Vec<Expr<'a, S>>;

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
pub struct Module<'a, S>
where S: ScopednessTypestate
    , S: 'a {
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

impl<'a> Scoped<'a, Module<'a, ScopedState>> {

    /// Returns true if the namespace contains a given name.
    pub fn contains_name<Q: ?Sized>(&self, name: &Q) -> bool
    where String: Borrow<Q>
        , String: PartialEq<Q>
        , Q: Hash + Eq {
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
  , App(AppForm<'a, S>)
  , Lambda(Function<'a, S>)
  , Logical(Logical<'a, S>)
  , Num(NumExpr<'a, S>)
  , Lit(Literal)
  , NameRef(NameRef)
}

/// AST node for a function application
#[derive(PartialEq, Clone, Debug)]
pub struct AppForm<'a, S>
where S: ScopednessTypestate
    , S: 'a { /// The name of the function being applied.
              ///
              /// Once we have constructed a complete list of defs (i.e.,
              /// once we are in the scoped state) we can check that this
              /// identifier is defined.
              pub fun: Ident
            , /// A list of parameters to the function application.
              ///
              /// Once we are in the scoped state we can check this for
              /// validity against the function's definition.
              pub params: Body<'a, S>
            }

impl<'a> Scoped<'a, AppForm<'a, ScopedState>> {

    /// Get the function definition for the function that is being applied
    pub fn get_fn_def(&self) -> CompileResult<&SymbolAnnotation> {
        let ref name = *(self.node.fun);
        self.symbol_table()
            .get(name)
            .ok_or(vec![self.map_pos(format!("Undefined function {}", name))])
    }

    /// Checks whether this call is valid for the function's definition
    pub fn is_valid(&self) -> CompileResult<()> {
        unimplemented!()
    }
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
             , fun: Annotated<'a, Function<'a, S>, S>
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

#[derive(PartialEq, Clone, Debug)]
pub enum Variant<'a, S>
where S: ScopednessTypestate
    , S: 'a { /// A variant that is only a tagword
              Tagword(Ident)
            , /// A variant with a constant value
              Constant(Literal)
            , /// A variant that is a record type
              Record(Vec<Annotated<'a, Formal, S>>)
            , /// A variant that is a single value
              Value(types::Type)
            , /// A variant that is itself a sum type
              Sum(HashMap<Ident, Variant<'a, S>>)
            }


#[derive(PartialEq, Clone, Debug)]
pub struct Data<'a, S>
where S: ScopednessTypestate
    , S: 'a { pub name: Ident
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
    #[inline] pub fn is_sum_type(&self) -> bool {
        self.variants.len() > 1
    }

    /// Returns true if this ADT is a product type
    #[inline]
    pub fn is_struct(&self) -> bool {
        !self.is_sum_type() &&
        self.get_struct_fields()
            .is_some()
    }

    /// Returns the fields of a product type.
    ///
    /// # Returns
    ///  + A `Some<Vec<Annotated<'a, Formal, S>>>` representing
    ///    this type's fields, if this is the definition of a
    ///    product type.
    ///  + Otherwise, `None`
    #[inline]
    pub fn get_struct_fields(&self) -> Option<&Vec<Annotated<'a, Formal, S>>> {
        self.variants
            .get(&self.name)
            .and_then(|var| match var { &Variant::Record(ref fs) => Some(fs)
                                      , _                        => None
                                      })

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
         , Form::App(ref form)=>
               format!( "({} {})"
                      , form.fun.to_sexpr(level)
                      , concat_exprs!(form.params, level) )
         , Form::Lambda(ref fun)   => fun.to_sexpr(level)
         , Form::Logical(ref form) => form.to_sexpr(level)
         , Form::Lit(ref c)   => format!("{}", c)
         , Form::NameRef(ref n)    => n.to_sexpr(level)
         , Form::Num(ref n) => unimplemented!()
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
    , S: 'a {
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
        match *self  { NameRef::Owned(ref name)    => format!("{}", **name)
                     , NameRef::Borrowed(ref name) => format!("&{}", **name)
                     , NameRef::Deref(ref name)    => format!("${}", **name)
                     , NameRef::Unique(ref name)   => format!("@{}", **name)
                     }

    }
}

impl Node for Ident {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

}
#[derive(PartialEq, Clone, Debug)]
pub enum NumExpr<'a, S>
where S: ScopednessTypestate
    , S: 'a { BOp(NumBOp<'a, S>)
            , Neg(Box<NumExpr<'a, S>>)
            , Lit(Literal)
            , Deref(NameRef)
            , Call(AppForm<'a, S>)
            }


/// Type for binary operators that yield a number
#[derive(PartialEq, Clone, Debug)]
pub enum NumBOp<'a, S>
where S: ScopednessTypestate
    , S: 'a { Add(Vec<NumExpr<'a, S>>)
            , Sub(Vec<NumExpr<'a, S>>)
            , Mul(Vec<NumExpr<'a, S>>)
            , Div(Vec<NumExpr<'a, S>>)
            , BitAnd(Vec<NumExpr<'a, S>>)
            , BitOr(Vec<NumExpr<'a, S>>)
            , BitXor(Vec<NumExpr<'a, S>>)
            , ShiftL(Vec<NumExpr<'a, S>>)
            , ShiftR(Vec<NumExpr<'a, S>>)
            }

trait MaybeConst {
    fn is_const(&self) -> bool;
}

impl<'a, S> MaybeConst for NumExpr<'a, S>
where S: ScopednessTypestate {

    #[inline] fn is_const(&self) -> bool {
        match self {
            &NumExpr::Lit(_)       => true
          , &NumExpr::Neg(ref neg) => neg.is_const()
          , &NumExpr::BOp(ref bop) => bop.is_const()
          , _ => false
        }
    }
}

impl<'a, S> MaybeConst for Vec<NumExpr<'a, S>>
where S: ScopednessTypestate {

    #[inline] fn is_const(&self) -> bool {
        self.iter().all(|expr| match expr {
            &NumExpr::Lit(_)       => true
          , &NumExpr::Neg(ref neg) => neg.is_const()
          , &NumExpr::BOp(ref bop) => bop.is_const()
          , _ => false
        })
    }
}

impl<'a, S> MaybeConst for NumBOp<'a, S>
where S: ScopednessTypestate {

    #[inline] fn is_const(&self) -> bool {
        match self { &NumBOp::Add(ref operands) => operands.is_const()
                   , &NumBOp::Sub(ref operands) => operands.is_const()
                   , &NumBOp::Mul(ref operands) => operands.is_const()
                   , &NumBOp::Div(ref operands) => operands.is_const()
                   , &NumBOp::BitAnd(ref operands) => operands.is_const()
                   , &NumBOp::BitOr(ref operands) => operands.is_const()
                   , &NumBOp::BitXor(ref operands) => operands.is_const()
                   , &NumBOp::ShiftL(ref operands) => operands.is_const()
                   , &NumBOp::ShiftR(ref operands) => operands.is_const()
                   }
    }
}


impl<'a> NumExpr<'a, UnscopedState> {

    pub fn new(op: String, exps: Vec<NumExpr<'a, UnscopedState>>) -> Self {
        match op.as_ref() {
            "+" if exps.is_const() => unimplemented!()
            , _ => unimplemented!()
        }
    }
}

pub enum BoolBOp<'a, S>
where S: ScopednessTypestate
    , S: 'a { Lt(Expr<'a, S>, Expr<'a, S>)
            , LtE(Expr<'a, S>, Expr<'a, S>)
            , Gt(Expr<'a, S>, Expr<'a, S>)
            , GtE(Expr<'a, S>, Expr<'a, S>)
            , Equal(Expr<'a, S>, Expr<'a, S>)
            , NEqual(Expr<'a, S>, Expr<'a, S>)
            , And(Expr<'a, S>, Expr<'a, S>)
            , Or(Expr<'a, S>, Expr<'a, S>)
            }
