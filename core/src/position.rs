use std::ops::{Deref, DerefMut};
use std::fmt;

use std::convert::From;

use combine::primitives::SourcePosition;

/// Struct representing a position within a source code file.
///
/// This represents positions using `i32`s because that's how
/// positions are represented in `combine` (the parsing library
/// that we will use for the Mnemosyne parser). I personally would
/// have used `usize`s...
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub col: i32,
    pub row: i32,
    pub raw: i32
}

impl Position {

    /// Create a new `Position `at the given column and row.
    #[inline]
    pub fn new(col: i32, row: i32) -> Self {
        Position { col: col
                 , row: row
                 , raw: col + row
                 }
    }

}

impl From<SourcePosition> for Position {
    /// Create a new `Position` from a `combine` `SourcePosition`.
    ///
    /// # Example
    /// ```
    /// let sp = SourcePosition { column: 1, line: 1 };
    /// assert_eq!(Position::from(sp), Position::new(1,1));
    /// ```
    fn from(p: SourcePosition) -> Self { Position::new(p.column, p.line) }
}

impl From<(i32,i32)> for Position {
    /// Create a new `Position` from a tuple of i32s.
    ///
    /// # Example
    /// ```
    /// let tuple: (i32,i32) = (1,1);
    /// assert_eq!(Position::from(tuple), Position::new(1,1));
    /// ```
    fn from((col, row): (i32,i32)) -> Self { Position::new(col,row) }
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

impl<T> Positional<T> {
    /// Create a new Positional marker at the given position.
    pub fn at(col: i32, row: i32, value: T) -> Positional<T> {
        Positional { pos: Position::new(col, row)
                   , value: value }
    }

    pub fn value(&self) -> &T { &self.value }
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
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
