/// *****************************************************************************
/// R-Graphs - A simple graph library for Rust
/// Copyright (C) 2016-2017 J. Férard <https://github.com/jferard>
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
use std::collections::HashSet;
use std::collections::hash_map::Iter;
use std::cmp::Eq;
use std::hash::Hash;

use util::edge_set::EdgeSet;
use util::iterator_util::HashMapHelper;

/// An EdgeSet that accepts multiple edges between two vertices.
pub struct MultipleEdgeSet<V, E> {
    edges_by_to_by_from: HashMap<V, HashMap<V, HashSet<E>>>,
    helper: HashMapHelper<V, HashSet<E>>
}

impl<V, E> EdgeSet<V, E> for MultipleEdgeSet<V, E>
    where V: Eq + Hash,
          E: Eq + Hash
{
    type S = HashSet<E>;

    fn new() -> Self {
        MultipleEdgeSet {
            edges_by_to_by_from: HashMap::new(),
            helper: HashMapHelper::new(),
        }
    }

    /// Add e to the set of vertices between u and v
    fn add_edge(&mut self, u: V, v: V, e: E) -> bool {
        let edges_by_to = self.edges_by_to_by_from.entry(u).or_insert(HashMap::new());
        let edges = edges_by_to.entry(v).or_insert(HashSet::new());
        edges.insert(e)
    }

    /// Remove the edge e from u to v, and return true if the edge was removed
    fn remove_edge(&mut self, u: &V, v: &V, e: &E) -> bool {
        let mut len;
        let ret;
        match self.edges_by_to_by_from.get_mut(u) {
            Some(mut edges_by_to) => {
                match edges_by_to.get_mut(v) {
                    Some(mut edges) => {
                        ret = edges.remove(e);
                        len = edges.len();
                    }
                    None => {
                        return false;
                    }
                };
                // lets clean edges_by_to : no edge goes from u to v
                if ret && len == 0 {
                    edges_by_to.remove(v);
                    len = edges_by_to.len();
                }
            }
            None => {
                return false;
            }
        }
        // lets clean edges_by_to_by_from : no edge starts from u anymore
        if ret && len == 0 {
            self.edges_by_to_by_from.remove(u);
        }
        ret
    }

    fn edges_by_to_by_from_iter(&self) -> Iter<V, HashMap<V, HashSet<E>>> {
        self.edges_by_to_by_from.iter()
    }

    fn edges_by_to_iter(&self, u: &V) -> Iter<V, HashSet<E>> {
        match self.edges_by_to_by_from.get(u) {
            Some(m) => m.iter(),
            None => self.helper.empty(),
        }
    }

    fn get_edges(&self, u: &V, v: &V) -> Option<&HashSet<E>> {
        match self.edges_by_to_by_from.get(u) {
            Some(edges_by_to) => {
                edges_by_to.get(v)
            }
            None => Option::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util::edge_set::EdgeSet;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_add_twice() {
        let mut set = MultipleEdgeSet::new();
        assert!(set.add_edge(1, 2, 0));
        assert!(set.add_edge(1, 2, 0) == false);
        assert!(set.add_edge(1, 2, 1));
    }

    #[test]
    fn test_remove_none() {
        let mut set = MultipleEdgeSet::new();
        assert!(set.remove_edge(&1, &2, &0) == false);
    }

    #[test]
    fn test_add_remove() {
        let mut s = HashSet::new();
        s.insert(0);
        let mut m = HashMap::new();
        m.insert(2, s);

        let mut set = MultipleEdgeSet::new();
        assert!(set.edges_by_to_by_from_iter().next().is_none());
        set.add_edge(1, 2, 0);
        {
            let mut i = set.edges_by_to_by_from_iter();
            //        assert!(i.next().unwrap() == (&x, &2, &s));
            assert!(i.next().is_some());
            assert!(i.next().is_none());
        }
        set.remove_edge(&1, &2, &0);
        assert!(set.edges_by_to_by_from_iter().next().is_none());
    }
}
