//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//

use ::errors::ExpectICE;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::{Keys,Values};
use std::hash::Hash;
use std::borrow::Borrow;
use std::ops;

/// An associative map data structure for representing scopes.
///
/// A `ForkTable` functions similarly to a standard associative map
/// data structure (such as a `HashMap`), but with the ability to
/// fork children off of each level of the map. If a key exists in any
/// of a child's parents, the child will 'pass through' that key. If a
/// new value is bound to a key in a child level, that child will overwrite
/// the previous entry with the new one, but the previous `key` -> `value`
/// mapping will remain in the level it is defined. This means that the parent
/// level will still provide the previous value for that key.
///
/// This is an implementation of the ForkTable data structure for
/// representing scopes. The ForkTable was initially described by
/// Max Clive. This implemention is based primarily by the Scala
/// reference implementation written by Hawk Weisman for the Decaf
/// compiler, which is available [here](https://github.com/hawkw/decaf/blob/master/src/main/scala/com/meteorcode/common/ForkTable.scala).
#[derive(Debug, Clone)]
pub struct ForkTable<'a, K, V>
where K: Eq + Hash
    , K: 'a
    , V: 'a
{
    table: HashMap<K, V>
  , whiteouts: HashSet<K>
  , parent: Option<&'a ForkTable<'a, K, V>>
  , level: usize
}

impl<'a, K, V> ForkTable<'a, K, V>
where K: Eq + Hash
{

    /// Returns a reference to the value corresponding to the key.
    ///
    /// If the key is defined in this level of the table, or in any
    /// of its' parents, a reference to the associated value will be
    /// returned.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `key`  - the key to search for
    ///
    /// # Return Value
    ///
    ///  + `Some(&V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1), None);
    /// table.insert(1, "One");
    /// assert_eq!(table.get(&1), Some(&"One"));
    /// assert_eq!(table.get(&2), None);
    /// ```
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1, "One");
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.get(&1), Some(&"One"));
    /// ```
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q>
        , Q: Hash + Eq
    {
        if self.whiteouts.contains(key) {
            None
        } else {
            self.table
                .get(key)
                .or(self.parent
                        .map_or(None, |ref parent| parent.get(key))
                    )
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// If the key is defined in this level of the table, a reference to the
    /// associated value will be returned.
    ///
    /// Note that only keys defined in this level of the table can be accessed
    /// as mutable. This is because otherwise it would be necessary for each
    /// level of the table to hold a mutable reference to its parent.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `key`  - the key to search for
    ///
    /// # Return Value
    ///
    ///  + `Some(&mut V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get_mut(&1), None);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.get_mut(&1), Some(&mut "One"));
    /// assert_eq!(table.get_mut(&2), None);
    /// ```
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1, "One");
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.get_mut(&1), None);
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where K: Borrow<Q>
        , Q: Hash + Eq
    {
        self.table.get_mut(key)
    }


    /// Removes a key from the map, returning the value at the key if
    /// the key was previously in the map.
    ///
    /// If the removed value exists in a lower level of the table,
    /// it will be whited out at this level. This means that the entry
    /// will be 'removed' at this level and this table will not provide
    /// access to it, but the mapping will still exist in the level where
    /// it was defined. Note that the key will not be returned if it is
    /// defined in a lower level of the table.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `key`  - the key to remove
    ///
    /// # Return Value
    ///
    ///  + `Some(V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// table.insert(1, "One");
    ///
    /// assert_eq!(table.remove(&1), Some("One"));
    /// assert_eq!(table.contains_key(&1), false);
    /// ```
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1, "One");
    /// assert_eq!(level_1.contains_key(&1), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1), true);
    /// assert_eq!(level_2.remove(&1), None);
    /// assert_eq!(level_2.chain_contains_key(&1), false);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V>
    where K: Clone
    {
        self.whiteouts.insert(key.clone());
        self.table.remove(&key)
    }

    /// Removes a key from this layer's map and whiteouts, so that
    /// definitions of that key from lower levels are exposed.
    ///
    /// Unlike `ForkTable::remove()`, if the removed value exists in a
    /// lower level of the table, it will NOT be whited out. This means
    /// that the definition of that entry from lower levels of the table
    /// will be exposed at this level.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `key`  - the key to expose
    ///
    /// # Return Value
    ///
    ///  + `Some(V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    pub fn expose<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where K: Borrow<Q>
        , Q: Hash + Eq
    {
        self.whiteouts.remove(key);
        self.table.remove(key)
    }

    /// Inserts a key-value pair from the map.
    ///
    /// If the key already had a value present in the map, that
    /// value is returned. Otherwise, `None` is returned.
    ///
    /// If the key is currently whited out (i.e. it was defined
    /// in a lower level of the map and was removed) then it will
    /// be un-whited out and added at this level.
    ///
    /// # Arguments
    ///
    ///  + `k`  - the key to add
    ///  + `v`  - the value to associate with that key
    ///
    /// # Return Value
    ///
    ///  + `Some(V)` if a previous entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// Simply inserting an entry:
    ///
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1), None);
    /// table.insert(1, "One");
    /// assert_eq!(table.get(&1), Some(&"One"));
    /// ```
    ///
    /// Overwriting the value associated with a key:
    ///
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1), None);
    /// assert_eq!(table.insert(1, "one"), None);
    /// assert_eq!(table.get(&1), Some(&"one"));
    ///
    /// assert_eq!(table.insert(1, "One"), Some("one"));
    /// assert_eq!(table.get(&1), Some(&"One"));
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.whiteouts.contains(&k) {
            self.whiteouts.remove(&k);
        };
        self.table.insert(k, v)
    }

    /// Returns true if this level contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `k`  - the key to search for
    ///
    /// # Return Value
    ///
    ///  + `true` if the given key is defined in this level of the
    ///    table, `false` if it does not.
    ///
    /// # Examples
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.contains_key(&1), false);
    /// table.insert(1, "One");
    /// assert_eq!(table.contains_key(&1), true);
    /// ```
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.contains_key(&1), false);
    /// level_1.insert(1, "One");
    /// assert_eq!(level_1.contains_key(&1), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.contains_key(&1), false);
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where K: Borrow<Q>
        , Q: Hash + Eq
    {
        !self.whiteouts.contains(key) &&
         self.table.contains_key(key)
    }

    /// Returns true if the key is defined in this level of the table, or
    /// in any of its' parents and is not whited out.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `k`  - the key to search for
    ///
    /// # Return Value
    ///
    ///  + `true` if the given key is defined in the table,
    ///    `false` if it does not.
    ///
    /// # Examples
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.chain_contains_key(&1), false);
    /// table.insert(1, "One");
    /// assert_eq!(table.chain_contains_key(&1), true);
    /// ```
    /// ```ignore
    /// # use mnemosyne::forktable::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.chain_contains_key(&1), false);
    /// level_1.insert(1, "One");
    /// assert_eq!(level_1.chain_contains_key(&1), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1), true);
    /// ```
    pub fn chain_contains_key<Q:? Sized>(&self, key: &Q) -> bool
    where K: Borrow<Q>
       , Q: Hash + Eq
    {
        self.table.contains_key(key) ||
            (!self.whiteouts.contains(key) &&
                self.parent
                    .map_or(false, |ref p| p.chain_contains_key(key))
                )
    }

    /// Forks this table, returning a new `ForkTable<K,V>`.
    ///
    /// This level of the table will be set as the child's
    /// parent. The child will be created with an empty backing
    /// `HashMap` and no keys whited out.
    ///
    /// Note that the new `ForkTable<K,V>` has a lifetime
    /// bound ensuring that it will live at least as long as the
    /// parent `ForkTable`.
    pub fn fork(&'a self) -> ForkTable<'a, K, V> {
        ForkTable { table: HashMap::new()
                  , whiteouts: HashSet::new()
                  , parent: Some(self)
                  , level: self.level + 1
                  }
    }

    /// Constructs a new `ForkTable<K,V>`
    pub fn new() -> ForkTable<'a, K,V> {
        ForkTable { table: HashMap::new()
                  , whiteouts: HashSet::new()
                  , parent: None
                  , level: 0
                  }
    }

    /// Wrapper for the backing map's `values()` function.
    ///
    /// Provides an iterator visiting all values in arbitrary
    /// order. Iterator element type is &'b V.
    pub fn values(&self) -> Values<K, V> { self.table.values() }

    /// Wrapper for the backing map's `keys()` function.
    ///
    /// Provides an iterator visiting all keys in arbitrary
    /// order. Iterator element type is &'b K.
    pub fn keys(&self) -> Keys<K, V> { self.table.keys() }
}

/// Allows `table[&key]` indexing syntax.
///
/// This is just a wrapper for `get(&key)`
///
/// ```ignore
/// # use mnemosyne::forktable::ForkTable;
/// let mut table: ForkTable<isize,&str> = ForkTable::new();
/// table.insert(1, "One");
/// assert_eq!(table[&1], "One");
/// ```
impl<'a, 'b, K, Q: ?Sized, V> ops::Index<&'b Q> for ForkTable<'a, K, V>
where K: Borrow<Q>
    , K: Eq + Hash
    , Q: Eq + Hash
{
    type Output = V;

    #[inline]
    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index)
            .expect_ice("undefined index")
    }

}

/// Allows mutable `table[&key]` indexing syntax.
///
/// This is just a wrapper for `get_mut(&key)`
///
/// ```ignore
/// # use mnemosyne::forktable::ForkTable;
/// let mut table: ForkTable<isize,&str> = ForkTable::new();
/// table.insert(1, "One");
/// table[&1] = "one";
/// assert_eq!(table[&1], "one")
/// ```
impl<'a, 'b, K, Q: ?Sized, V> ops::IndexMut<&'b Q> for ForkTable<'a, K, V>
where K: Borrow<Q>
    , K: Eq + Hash
    , Q: Eq + Hash
{
    #[inline]
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index)
            .expect_ice("undefined index")
    }

}

#[cfg(test)]
mod tests {
    use super::ForkTable;

    #[test]
    fn test_get_defined() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.get(&1), None);
        table.insert(1, "One");
        assert_eq!(table.get(&1), Some(&"One"));
    }

    #[test]
    fn test_get_undefined() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        assert_eq!(table.get(&2), None);
    }
    #[test]
    fn test_get_multilevel() {
        let mut level_1: ForkTable<isize,&str> = ForkTable::new();
        level_1.insert(1, "One");

        let mut level_2: ForkTable<isize,&str> = level_1.fork();
        assert_eq!(level_2.get(&1), Some(&"One"));
    }

    #[test]
    fn test_get_mut_defined() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.get_mut(&1), None);
        table.insert(1, "One");
        assert_eq!(table.get_mut(&1), Some(&mut "One"));
    }

    #[test]
    fn test_get_mut_undefined() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        assert_eq!(table.get_mut(&2), None);
    }
    #[test]
    fn test_get_mut_multilevel() {
        let mut level_1: ForkTable<isize,&str> = ForkTable::new();
        level_1.insert(1, "One");

        let mut level_2: ForkTable<isize,&str> = level_1.fork();
        assert_eq!(level_2.get_mut(&1), None);
    }
    #[test]
    fn test_remove_returned() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        assert_eq!(table.remove(&1), Some("One"));
    }
    #[test]
    fn test_remove_not_defined_after() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        table.remove(&1);
        assert_eq!(table.get(&1), None);
    }

    #[test]
    fn test_remove_multilevel() {
        let mut level_1: ForkTable<isize,&str> = ForkTable::new();
        level_1.insert(1, "One");
        assert_eq!(level_1.contains_key(&1), true);

        let mut level_2: ForkTable<isize,&str> = level_1.fork();
        assert_eq!(level_2.chain_contains_key(&1), true);
        assert_eq!(level_2.remove(&1), None);
        assert_eq!(level_2.chain_contains_key(&1), false);
    }

    #[test]
    fn test_insert_defined_after() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.get(&1), None);
        table.insert(1, "One");
        assert_eq!(table.get(&1), Some(&"One"));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.get(&1), None);
        assert_eq!(table.insert(1, "one"), None);
        assert_eq!(table.get(&1), Some(&"one"));

        assert_eq!(table.insert(1, "One"), Some("one"));
        assert_eq!(table.get(&1), Some(&"One"));
    }

    #[test]
    fn test_contains_key() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.contains_key(&1), false);
        table.insert(1, "One");
        assert_eq!(table.contains_key(&1), true);
    }

    #[test]
    fn test_contains_key_this_level_only () {
        let mut level_1: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(level_1.contains_key(&1), false);
        level_1.insert(1, "One");
        assert_eq!(level_1.contains_key(&1), true);

        let mut level_2: ForkTable<isize,&str> = level_1.fork();
        assert_eq!(level_2.contains_key(&1), false);
    }

    #[test]
    fn test_chain_contains_key_this_level() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(table.chain_contains_key(&1), false);
        table.insert(1, "One");
        assert_eq!(table.chain_contains_key(&1), true);
    }

    #[test]
    fn test_contains_key_multilevel() {
        let mut level_1: ForkTable<isize,&str> = ForkTable::new();
        assert_eq!(level_1.chain_contains_key(&1), false);
        level_1.insert(1, "One");
        assert_eq!(level_1.chain_contains_key(&1), true);

        let mut level_2: ForkTable<isize,&str> = level_1.fork();
        assert_eq!(level_2.chain_contains_key(&1), true);
    }

    #[test]
    fn test_indexing() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        assert_eq!(table[&1], "One");
    }

    #[test]
    fn test_index_mut() {
        let mut table: ForkTable<isize,&str> = ForkTable::new();
        table.insert(1, "One");
        table[&1] = "one";
        assert_eq!(table[&1], "one")
    }
}
