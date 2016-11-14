/// *****************************************************************************
/// R-Graphs - A simple graph library for Rust
/// Copyright (C) 2016 J. Férard <https://github.com/jferard>
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
use std::collections::hash_map::Entry;
use std::fmt::{self, Debug, Formatter};
use util::disjoint_set::ValueDisjointSet;

// Sedgewick, chapter 30
// CLRS, 21.1

pub struct UsizeTreeDisjointSet {
    parent: Vec<usize>, // parent of node
    rank: Vec<usize>, // to keep the tree balanced
}

impl UsizeTreeDisjointSet {
    fn find_root(&self, x: usize) -> usize {
        if x >= self.parent.len() {
            panic!();
        }
        let mut y = x;
        let mut p = self.parent[y];
        while y != p {
            if p >= self.parent.len() {
                panic!();
            }
            y = p;
            p = self.parent[p];
        }
        return p;
    }

    fn reduce_path(&mut self, x: usize, root: usize) {
        if x >= self.parent.len() {
            panic!();
        }
        let mut y = x;
        while y != root {
            if y >= self.parent.len() {
                panic!();
            }
            let p = self.parent[y];
            self.parent[y] = root;
            y = p;
        }
    }

    fn get_ranks(&self, x: usize, y: usize) -> (usize, usize) {
        assert!(self.parent.len() == self.rank.len());
        if x >= self.parent.len() || y >= self.parent.len() {
            panic!();
        }
        (self.rank[x], self.rank[y])
    }
}

impl ValueDisjointSet<usize> for UsizeTreeDisjointSet {
    fn new() -> UsizeTreeDisjointSet {
        UsizeTreeDisjointSet {
            parent: Vec::new(),
            rank: Vec::new(),
        }
    }

    fn add_set(&mut self, x: usize) -> bool {
        self.parent.push(x);
        self.rank.push(0);
        true
    }

    fn find(&mut self, x: usize) -> Option<usize> {
        let r = self.find_root(x);
        self.reduce_path(x, r);
        Some(r)
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x1 = {
            self.find(x).unwrap()
        };
        let y1 = {
            self.find(y).unwrap()
        };
        let (rank_x, rank_y) = self.get_ranks(x1, y1);
        if rank_x < rank_y {
            self.parent[x1] = y1;
        } else if rank_y < rank_x {
            self.parent[y1] = x1;
        } else {
            self.parent[x1] = y1;
            self.rank[x1] = rank_x + 1;
        }
        true
    }
}

impl UsizeTreeDisjointSet {
    pub fn new(x: usize) -> UsizeTreeDisjointSet {
        let mut parent = Vec::with_capacity(x);
        let mut rank = Vec::with_capacity(x);
        for i in 0..x {
            parent.push(i);
            rank.push(0);
        }
        UsizeTreeDisjointSet {
            parent: parent,
            rank: rank,
        }
    }
}

impl Debug for UsizeTreeDisjointSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut r = HashMap::new();
        for (element, parent) in self.parent.iter().enumerate() {
            match r.entry(parent) { // borrows &mut self and u
                Entry::Vacant(ve) => {
                    ve.insert(vec![element]);
                }
                Entry::Occupied(mut oe) => {
                    let mut children = oe.get_mut();
                    children.push(element);
                }
            }
        }
        for (parent, children) in r {
            try!(write!(f, "{:?} -> {:?}\n", parent, children));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util::disjoint_set::ValueDisjointSet;

    #[test]
    fn test_usize_tree_disjoint_set() {
        let mut x = UsizeTreeDisjointSet::new(10);
        for i in 0..10 {
            assert!(x.parent[i] == i);
        }
        x.union(1, 2);
        assert!(x.parent[1] == x.parent[2]);
        assert!([1, 2].contains(&x.parent[1]));
        for i in (0..10).filter(|&j| ![1, 2].contains(&j)) {
            assert!(x.parent[i] == i);
        }
        x.union(6, 7);
        assert!(x.parent[1] == x.parent[2]);
        assert!([1, 2].contains(&x.parent[1]));
        assert!(x.parent[6] == x.parent[7]);
        assert!([6, 7].contains(&x.parent[6]));
        for i in (0..10).filter(|&j| ![1, 2, 6, 7].contains(&j)) {
            assert!(x.parent[i] == i);
        }
        x.union(1, 6);
        assert!(x.find(1) == x.find(6));
        assert!(x.parent[1] == x.parent[2]);
        assert!(x.parent[2] == x.parent[6]);
        assert!(x.parent[6] == x.parent[7]);
        assert!([1, 2, 6, 7].contains(&x.parent[1]));
        for i in (0..10).filter(|&j| ![1, 2, 6, 7].contains(&j)) {
            assert!(x.parent[i] == i);
        }
    }
}
