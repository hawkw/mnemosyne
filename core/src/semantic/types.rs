use super::ASTNode;
use seax::compiler_tools::ForkTable;

pub struct Scoped<'a, T> where T: ASTNode {
    node: T, scope: ForkTable<'a, String, Annotation>
}

pub struct Annotation;
