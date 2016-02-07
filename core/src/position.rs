//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Source code position implementation

use std::ops::{Deref, DerefMut};
use std::hash;
use std::fmt;
use std::convert::From;

use combine::primitives::SourcePosition;

/// Struct representing a position within a source code file.
///
/// This represents positions using `i32`s because that's how
/// positions are represented in `combine` (the parsing library
/// that we will use for the Mnemosyne parser). I personally would
/// have used `usize`s...
#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Position { pub col: i32
                    , pub row: i32
                    , pub raw: i32
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
    /// # extern crate combine;
    /// # extern crate mnemosyne;
    /// # use combine::primitives::SourcePosition;
    /// # use mnemosyne::position::Position;
    /// # fn main() {
    /// let sp = SourcePosition { column: 1, line: 1 };
    /// assert_eq!(Position::from(sp), Position::new(1,1));
    /// # }
    /// ```
    fn from(p: SourcePosition) -> Self { Position::new(p.column, p.line) }
}

impl From<(i32,i32)> for Position {
    /// Create a new `Position` from a tuple of i32s.
    ///
    /// # Example
    /// ```
    /// # use mnemosyne::position::Position;
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

/// A pointer to a value with an associated `Position`
#[derive(Clone, Debug)]
pub struct Positional<T> { pub pos: Position
                         , pub value: T
                         }

impl<A> Positional<A> {

    /// Create a new Positional marker at the given position.
    #[inline]
    pub fn at(col: i32, row: i32, value: A) -> Positional<A> {
        Positional { pos: Position::new(col, row)
                   , value: value }
    }

    #[inline]
    pub fn from(pos: Position, value: A) -> Positional<A> {
        Positional { pos: pos, value: value }
    }

    /// Create a Positional wrapping a new value at the same position
    #[inline]
    pub fn map<B>(&self, value: B) -> Positional<B> {
        Positional { pos: self.pos
                   , value: value }
    }

    pub fn value(&self) -> &A { &self.value }
}


impl<T> fmt::Display for Positional<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.value, self.pos)
    }
}

/// A positional pointer is still equal to the underlying
/// value even if they have different positions. This is
/// important so that we can test that two identifiers are
/// the same.
impl<T> PartialEq for Positional<T>
where T: PartialEq {
    fn eq(&self, other: &Positional<T>) -> bool {
        self.value == other.value
    }
}

/// If two things are equal, then they better have the same
/// hash as well. Otherwise there will be sadness.
///
/// Homefully this is Ideologically Correct.
impl<T> hash::Hash for Positional<T>
where T: hash::Hash {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}


/// This is literally just waving my hands for the compiler.
///
/// Hopefully it understands what I mean.
impl<T> Eq for Positional<T>
where T: Eq
    , T: PartialEq
    {}

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

#[cfg(test)]
mod tests {
    use super::*;
    use combine::primitives::SourcePosition;

    #[test]
    fn test_from_sourceposition() {
        let sp = SourcePosition { column: 1, line: 1 };
        assert_eq!(Position::from(sp), Position::new(1,1));
    }

    #[test]
    fn test_from_tuple() {
        let tuple: (i32,i32) = (1,1);
        assert_eq!(Position::from(tuple), Position::new(1,1));
    }
}
