//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//

use std::rc::Rc;
use std::fmt;
use std::fmt::Write;
use std::iter;

use ast;
use ::chars;

#[derive(Debug, Clone, PartialEq)]
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
    Function(Signature),
    /// A unique symbol type (`'symbol` syntax)
    Symbol(String)
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { &Type::Ref(ref r) =>  write!(f, "{}", r)
                   , &Type::Prim(ref p) => write!(f, "{}", p)
                   , &Type::Algebraic(ref variants) =>
                        unimplemented!()
                   , &Type::Function(ref fun) => write!(f, "{}", fun)
                   , &Type::Symbol(ref s) => write!(f, "{}", s)
                   }
    }
}

/// A function signature
///
/// This implements both `Type` and `Node`, since it can be used as
/// a type value (for a higher-order function), or as the AST node
/// for the definition of a function.
#[derive(Clone, Debug, PartialEq)]
pub struct Signature { /// Any typeclass constraints on the function
                       pub constraints: Option<Vec<Constraint>>
                     , /// The actual function type chain globule
                       pub typechain: Vec<Type>
                     }
impl Signature {
    /// The uncurried return type of the function
    ///
    /// This just returns the last element in the type glob
    pub fn return_type(&self) -> &Type { &self.typechain[0] }
    /// Returns the arity of the function
    pub fn arity(&self) -> usize { self.typechain.len() - 1 }
}

 impl fmt::Display for Signature {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         unimplemented!()
     }
 }

/// Format a bunch of things to Strings and concatenate them together.
fn concat_all<A,B>(xs: B) -> String
where A: fmt::Display
    , B: Iterator<Item=A>
{
    xs.fold(String::new(), |mut s, x| {
        write!(&mut s, "{}", x).expect("Could not append to string!");
        s
    })
}

 impl ast::Node for Signature {
     fn to_sexpr(&self, level: usize) -> String {
         format!( "{indent}{}({arrow} {})"
                , self.constraints
                      .clone()
                      .map(|ref cs| format!( "({arrow} {})"
                                      , concat_all(cs.iter())
                                      , arrow = chars::FAT_ARROW )
                        )
                    .unwrap_or(String::from(""))
                , concat_all(self.typechain.iter())
                , arrow  = chars::ARROW
                , indent = indent!(level)
                )
     }
 }

#[derive(Clone, Debug, PartialEq)]
pub struct Constraint { pub typeclass: ast::Ident
                      , pub generics: Vec<ast::Ident> }

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

/// Reference types (pointers)
///
/// TODO: how will lifetime analysis actually work?
#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self { Reference::Borrowed(ref t) =>  write!(f, "&{}", t)
                    , Reference::Moved(ref t) =>  write!(f, "move {}", t)
                    , Reference::Unique(ref t) =>  write!(f, "@{}", t)
                    , Reference::Raw(ref t) =>  write!(f, "*{}", t)
                    }
    }
}

/// Language primitive types
///
/// TODO: add some form of provable-refinement (i.e. we know that some value
/// is not just a bool at compile time but that it's true/false, or we know
/// some value is not just a number but the number 1382, or whatever).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Primitive { Int(Int)
                   , IntSize
                   , Uint(Int)
                   , UintSize
                   , Byte
                   , Char
                   , Str
                   , Bool
                   , Float
                   , Double
                   // TODO: finish
                   }
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Int { Int8  = 8
             , Int16 = 16
             , Int32 = 32
             , Int64 = 64
             }

impl fmt::Display for Primitive {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self { Primitive::Int(bits)  => write!(f, "i{}", bits as isize)
                   , Primitive::Uint(bits) => write!(f, "u{}", bits as isize)
                   , Primitive::IntSize    => write!(f, "int")
                   , Primitive::UintSize   => write!(f, "uint")
                   , Primitive::Double     => write!(f, "double")
                   , Primitive::Float      => write!(f, "float")
                   , Primitive::Bool       => write!(f, "bool")
                   , _ => unimplemented!()
                   }
   }
}
