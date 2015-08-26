use std::ops::{Deref, DerefMut};
use std::fmt;
use combine::primitives::SourcePosition;

/// Struct representing a position within a source code file
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub col: isize,
    pub row: isize,
    pub raw: isize
}

impl Position {

    fn new(col: isize, row: isize) -> Self {
        Position { col: col
                 , row: row
                 , raw: col + row
                 }
    }

    fn from_combine(pos: SourcePosition) -> Self {
        Position { col: pos.column
                 , row: pos.line
                 , raw: pos.column + pos.line
                 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.row, self.col)
    }
}

#[derive(Clone, Debug)]
pub struct Positional<T> {
    pub pos: Position,
    pub value: T
}

impl Positional {
    fn at<T>(col: isize, row: isize, value: T) -> Positional<T> {
        Positional { pos: Position::new(col, row)
                   , value: value }
    }
}


impl<T> fmt::Display for Positional<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.value, self.pos)
    }
}

impl<T> PartialEq for Positional<T>
where T: PartialEq {
    fn eq(&self, other: &Positional<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Deref for Positional<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Positional<T> {
    fn deref(&self) -> &mut T {
        &mut self.value
    }
}
