use ::position::Positional;

pub type Ident = Positional<String>;
pub type Expr = Box<Positional<Form>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Form {
    Define(DefForm),
    // If(IfNode),
    // Let(LetNode),
    // Call(CallNode)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident, pub annot: Ident }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm {
    TopLevel { name: Ident
             , annot: Ident
             , value: Expr
             },
    Function { name: Ident
             , annot: Ident
             , formals: Vec<Positional<Formal>>
             , body: Expr
             }
}
