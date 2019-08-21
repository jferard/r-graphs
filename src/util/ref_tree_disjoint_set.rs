use std::collections::hash_map::Entry;
/// *****************************************************************************
/// R-Graphs - A simple graph library for Rust
/// Copyright (C) 2016-2019 J. Férard <https://github.com/jferard>
///
/// This file is part of R-Graphs.
///
/// R-Graphs is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// R-Graphs is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.
/// ***************************************************************************
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

use util::disjoint_set::RefDisjointSet;

// DO NOT USE

pub struct RefTreeDisjointSet<'a, T>
    where T: 'a
{
    parent: HashMap<&'a T, &'a T>,
    // parent of node
    rank: HashMap<&'a T, usize>, // to keep the tree balanced
}

impl<'a, T> RefTreeDisjointSet<'a, T>
    where T: Clone + Eq + Hash + 'a
{
    fn find_root(&self, x: &'a T) -> Option<&'a T> {
        let mut y = x;
        let mut opt_parent_y = self.parent.get(y);
        if opt_parent_y.is_none() {
            return None;
        }
        let mut parent_y = *opt_parent_y.unwrap();
        while y != parent_y {
            y = parent_y;
            opt_parent_y = self.parent.get(parent_y);
            if opt_parent_y.is_none() {
                return None;
            }
            parent_y = *opt_parent_y.unwrap();
        }
        Some(parent_y)
    }

    /** Cleans the path from x to his parents. To use when x is disconnected from root. */
    fn remove_from_sets(&mut self, x: &'a T) -> Option<&'a T> {
        let mut y = x;
        loop {
            match self.parent.entry(y) {
                Entry::Vacant(_) => {
                    return None;
                }
                Entry::Occupied(oe) => {
                    y = oe.get();
                    oe.remove();
                }
            }
            match self.rank.entry(y) {
                Entry::Vacant(_) => {
                    panic!("rank and parent should have the same key set");
                }
                Entry::Occupied(oe) => {
                    oe.remove();
                }
            }
        }
    }

    /** reduce path from x to root, hanging x and his parents to root */
    fn reduce_path_to_root(&mut self, x: &'a T, root: &'a T) -> Option<&'a T> {
        let mut y = x;
        while y != root {
            match self.parent.entry(y) {
                Entry::Vacant(_) => {
                    panic!("y should reach root");
                }
                Entry::Occupied(mut oe) => {
                    y = oe.get();
                    oe.insert(root);
                }
            }
        }
        Some(root)
    }

    fn get_ranks(&mut self, x: &'a T, y: &'a T) -> Option<(&'a T, &'a T, usize, usize)> {
        let opt_root_x = {
            self.find(x)
        };
        let opt_root_y = {
            self.find(y)
        };
        match (opt_root_x, opt_root_y) {
            (Some(a), Some(b)) => {
                let root_x = a;
                let root_y = b;
                let rank_root_x = *self.rank.get(root_x).unwrap();
                let rank_root_y = *self.rank.get(root_y).unwrap();
                Some((root_x, root_y, rank_root_x, rank_root_y))
            }
            _ => None,
        }
    }
}

impl<'a, T: Clone + Eq + Hash> RefDisjointSet<'a, T> for RefTreeDisjointSet<'a, T> {
    fn new() -> RefTreeDisjointSet<'a, T> {
        let parent = HashMap::new();
        let rank = HashMap::new();
        RefTreeDisjointSet {
            parent,
            rank,
        }
    }

    fn add_set(&mut self, x: &'a T) -> bool {
        let op_is_none = self.parent.get(x).is_none();
        if op_is_none {
            self.parent.insert(x, x);
            self.rank.insert(x, 0);
        }
        op_is_none
    }

    fn find(&mut self, x: &'a T) -> Option<&'a T> {
        let optional_root: Option<&'a T> = self.find_root(x);
        match optional_root {
            None => self.remove_from_sets(x),
            Some(root) => self.reduce_path_to_root(x, &root),
        }
    }

    fn union(&mut self, x: &'a T, y: &'a T) -> bool {
        match self.get_ranks(x, y) {
            None => false,
            Some((root_x, root_y, rank_root_x, rank_root_y)) => {
                if rank_root_x < rank_root_y {
                    *self.parent.get_mut(root_x).unwrap() = &root_y;
                } else if rank_root_y < rank_root_x {
                    *self.parent.get_mut(root_y).unwrap() = &root_x;
                } else {
                    *self.parent.get_mut(root_x).unwrap() = &root_y;
                    *self.rank.get_mut(root_x).unwrap() = rank_root_x + 1;
                }
                true
            }
        }
    }
}

impl<'a, T: Clone + Eq + Hash> RefTreeDisjointSet<'a, T> {
    pub fn make_set(x: &'a T) -> RefTreeDisjointSet<'a, T> {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        parent.insert(x, x);
        rank.insert(x, 0);
        RefTreeDisjointSet {
            parent,
            rank,
        }
    }

    pub fn make_sets(vec: &'a Vec<T>) -> RefTreeDisjointSet<'a, T> {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for x in vec {
            parent.insert(x, x);
            rank.insert(x, 0);
        }
        RefTreeDisjointSet {
            parent,
            rank,
        }
    }
}

impl<'a, T: Clone + Eq + Hash + Debug> Debug for RefTreeDisjointSet<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut r = HashMap::new();
        for (element, parent) in &self.parent {
            match r.entry(parent) { // borrows &mut self and u
                Entry::Vacant(ve) => {
                    ve.insert(vec![element]);
                }
                Entry::Occupied(mut oe) => {
                    let children = oe.get_mut();
                    children.push(element);
                }
            }
        }
        for (parent, children) in r {
            write!(f, "{:?} -> {:?}\n", parent, children)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use util::disjoint_set::RefDisjointSet;

    use super::*;

    #[test]
    fn test_ref_tree_disjoint_set() {
        let v = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        let mut x = RefTreeDisjointSet::make_sets(&v);
        for e in &v {
            assert!(x.parent[e] == e);
        }
        let (aa, bb) = (&v[0], &v[1]);
        x.union(aa, bb);
        assert!(x.parent[aa] == x.parent[bb]);
        assert!(["a", "b"].contains(&&x.parent[bb]));
        let iterator = v.iter().filter(|j| !["a", "b"].contains(j)).cloned();
        for e in iterator {
            assert!(x.parent[&e] == &e);
        }
        x.union(&v[5], &v[6]);
        assert!(x.parent[&"a"] == x.parent[&v[1]]);
        assert!(["a", "b"].contains(&&x.parent[&v[0]]));
        assert!(x.parent[&"f"] == x.parent[&v[6]]);
        assert!(["f", "g"].contains(&&x.parent[&v[5]]));
        let mut iterator2 = v.iter().filter(|j| !["a", "b", "f", "g"].contains(j)).cloned();
        for e in iterator2.by_ref() {
            assert!(x.parent[&e] == &e);
        }
        x.union(&v[0], &v[5]);
        assert_eq!(x.find(&v[0]).cloned(), x.find(&v[5]).cloned());
        assert!(x.parent[&v[0]] == x.parent[&v[1]]);
        assert!(x.parent[&v[1]] == x.parent[&v[5]]);
        assert!(x.parent[&v[5]] == x.parent[&v[6]]);
        assert!(["a", "b", "f", "g"].contains(&&x.parent[&v[0]]));
        for e in iterator2 {
            assert!(x.parent[&e] == &e);
        }
    }
}
