use std::fmt;

pub trait ASTNode {
    /// Pretty-print this node at the desired indent level
    fn print_level(&self, level: usize) -> String;
}

impl fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print_level(0))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    // Def(DefNode),
    // If(IfNode),
    // Let(LetNode),
    // Call(CallNode)
}

impl ASTNode for Expression {
    fn print_level(&self, level: usize) -> String {
        unimplemented!()
    }
}
