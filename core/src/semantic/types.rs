use super::ASTNode;
use seax::compiler_tools::ForkTable;

pub struct Scoped<'a, T> where T: ASTNode {
    node: T, scope: ForkTable<'a, String, Type>
}

#[derive(Debug, Clone)]
pub enum Type { Ref(Reference)
              , Prim(Primitive)
              , Algebraic //TODO: make me a thing
              }

/// Reference types
///
/// TODO: how will lifetime analysis actually work?
#[derive(Debug, Clone)]
pub enum Reference { Borrowed(Box<Type>) // TODO: borrowed from where?
                   , Moved(Box<Type>) // TODO: moved from where?
                   , Owned(Box<Type>)
                   , Raw(Box<Type>)
                   }

#[derive(Debug, Clone, Copy)]
pub enum Primitive { Int, Uint, Byte, Str // TODO: finish
}
