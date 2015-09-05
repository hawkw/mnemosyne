use ::position::Positional;
use super::annotations::{ Annotated
                        , ScopednessTypestate
                        };
use super::types::Type;
use std::rc::Rc;

pub type Ident = Positional<String>;
pub type Expr<'a, S: ScopednessTypestate>
    = Rc<Annotated< 'a
                  , Form<'a, S>
                  , S>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Form<'a, S: ScopednessTypestate> {
    Define(DefForm<'a, S>)
  , If { condition: Expr<'a, S>
       , if_clause: Expr<'a, S>
       , else_clause: Option<Expr<'a, S>>
       }
  , Let { bindings: Vec<Annotated< 'a
                                 , Binding<'a, S>
                                 , S>>
        , body: Expr<'a, S>
        }
  , Call { fun: Ident
         , body: Vec<Expr<'a, S>>
         }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Formal { pub name: Ident
                  , pub annot: Ident
                  }

#[derive(PartialEq, Clone, Debug)]
pub enum DefForm<'a, S: ScopednessTypestate> {
    TopLevel { name: Ident
             , annot: Ident
             , value: Expr<'a, S>
             },
    Function { name: Ident
             , annot: Ident
             , formals: Vec<Annotated<'a, Formal, S>>
             , body: Expr<'a, S>
             }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Binding<'a, S: ScopednessTypestate> {
    pub name: Ident
  , pub typ: Type
  , pub value: Expr<'a, S>
}
