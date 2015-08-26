use std::ops::{Deref, DerefMut};
use std::fmt;

/// Struct representing a position within a source code file
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub col: isize,
    pub row: isize,
    pub raw: isize
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
