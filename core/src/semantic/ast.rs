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
pub type Body<'a, S: ScopednessTypestate> = Vec<Expr<'a, S>>;

pub type Bindings<'a, S: ScopednessTypestate>
    = Vec<Annotated< 'a
                   , Binding<'a, S>
                   , S>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Form<'a, S: ScopednessTypestate> {
    Define(DefForm<'a, S>)
  , If { condition: Expr<'a, S>
       , if_clause: Expr<'a, S>
       , else_clause: Option<Expr<'a, S>>
       }
  , Let(LetForm<'a, S>)
  , Call { fun: Ident
         , body: Body<'a, S>
         }
  , Lambda { formals: Vec<Annotated<'a, Formal, S>>
           , annot: Ident
           , body: Body<'a, S>
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
             }
  , Function { name: Ident
             , annot: Ident
             , formals: Vec<Annotated<'a, Formal, S>>
             , body: Body<'a, S>
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
  , pub typ: Type
  , pub value: Expr<'a, S>
}
