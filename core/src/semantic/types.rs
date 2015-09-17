use std::rc::Rc;
use std::fmt;

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
             , rt: Rc<Type>
             },
    /// A unique symbol type (`'symbol` syntax)
    Symbol(String)
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { &Type::Ref(ref r) =>  write!(f, "{}", r)
                   , &Type::Prim(ref p) => write!(f, "{}", p)
                   , &Type::Algebraic(ref variants) =>
                        unimplemented!()
                   , &Type::Function { ref params, ref rt } =>
                        unimplemented!()
                   , &Type::Symbol(ref s) => write!(f, "{}", s)
                   }
    }
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

impl fmt::Display for Primitive {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self { Primitive::Int     => write!(f, "int")
                   , Primitive::Uint    => write!(f, "uint")
                   , Primitive::Double  => write!(f, "double")
                   , Primitive::Float   => write!(f, "float")
                   , Primitive::Bool    => write!(f, "bool")
                   , _ => unimplemented!()
                   }
   }
}
