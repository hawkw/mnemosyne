use std::iter;
use std::fmt::Write;
use std::rc::Rc;

use ::forktable::ForkTable;

pub mod ast;
pub mod types;
pub mod annotations;

use ast::*;
use self::annotations::*;
use self::types::Type;

/// A symbol table is a `ForkTable` mapping `String`s to `Type` annotations.
///
/// This table should be forked upon entering a new scope.
pub type SymbolTable<'a>
    = ForkTable<'a, String, SymbolAnnotation<'a>>;

macro_rules! indent {
    ($to:expr) => ( iter::repeat('\t')
                        .take($to)
                        .collect::<String>() )
}

/// Trait for a node in the abstract syntax tree.
///
/// This provides a visitor method for semantic analysis (which may be split
/// into multiple transforms later), and a method for formatting the AST node
/// as an S-expression.
pub trait ASTNode: Sized {

    /// Pretty-print the AST node as an S-expression at the desired
    /// indentation level.
    ///
    /// Note that this prints the desugared form and may not be a perfect
    /// match for the original source code.
    fn to_sexpr(&self, level: usize) -> String;

}

pub trait AnnotateTypes<'a>: Sized {

    #[allow(unused_variables)]
    fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self> {
        unimplemented!() //TODO: implement
    }
}

impl<'a, S> ASTNode for Form<'a, S>
where S: ScopednessTypestate
{
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            Form::Define(ref form) => form.to_sexpr(level)
          , Form::Let(ref form) => form.to_sexpr(level)
          , Form::If { .. } => unimplemented!()
          , Form::Call { .. } => unimplemented!()
        }
    }

}

impl<'a> AnnotateTypes<'a> for Unscoped<'a, Form<'a, UnscopedState>> {
    #[allow(unused_variables)]
    fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self> {
        unimplemented!() //TODO: implement
    }
}

impl ASTNode for Ident {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String { self.value.clone() }

}

impl<'a, S> ASTNode for DefForm<'a, S>
where S: ScopednessTypestate {

    fn to_sexpr(&self, level: usize) -> String {
        match *self {
            DefForm::TopLevel { ref name, ref annot, ref value } =>
                format!("{}(define {} {} {})", indent!(level),
                    name.to_sexpr(level),
                    annot.to_sexpr(level),
                    value.to_sexpr(level+1)
                    ),
            DefForm::Function { ref name, ref annot, ref formals, ref body } =>
                format!("{}(define {} {} {}\n{})", indent!(level),
                    name.to_sexpr(level),
                    annot.to_sexpr(level),
                    formals.iter()
                           .fold(String::new(), |mut s, f| {
                                s.push_str(&f.to_sexpr(level)); s
                            }),
                    body.iter()
                        .fold(String::new(), |mut s, f| {
                                write!(s, "{}\n", &f.to_sexpr(level + 1)); s
                            }),
                    )
        }
    }

}

impl<'a, S> ASTNode for LetForm<'a, S>
where S: ScopednessTypestate {

    fn to_sexpr(&self, level: usize) -> String {
        unimplemented!()
    }

}

impl<'a> AnnotateTypes<'a> for Unscoped<'a, DefForm<'a, UnscopedState>> {
    #[allow(unused_variables)]
    fn annotate_types(self, scope: SymbolTable) -> Scoped<'a, Self>{
        unimplemented!() //TODO: implement
    }
}

impl ASTNode for Formal {
    #[allow(unused_variables)]
    fn to_sexpr(&self, level: usize) -> String {
        format!("{}: {}", *(self.name), *(self.annot))
    }

}

#[derive(Clone,Debug,PartialEq)]
pub struct SymbolAnnotation<'a> {
    /// The type of the symbol
    pub ty: Type,
    /// An optional proven value for the symbol.
    ///
    /// This should be defined iff the symbol signifies a constant value
    /// or a constant expression, or if we were able to prove that the value
    /// remains constant within the current scope.
    pub proven_value: Option<Rc<Expr<'a, ScopedState>>>
}
