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
use std::collections::hash_map;
use std::cmp::Eq;
use std::hash::Hash;

/// An EdgeSet is a set of edges.
/// One may add or remove an edge, and iterate on edges in various ways.
pub trait EdgeSet<V, E>
    where V: Eq + Hash,
          E: Eq + Hash
{
    type S; // Hashet<E> or E

    /// create a new EdgeSet
    fn new() -> Self;

    /// add an edge
    fn add_edge(&mut self, V, V, E) -> bool;

    /// remove an edge
    fn remove_edge(&mut self, &V, &V, &E) -> bool;

    /// return an iterator on maps vertex -> set
    fn edges_by_to_by_from_iter(&self) -> hash_map::Iter<V, HashMap<V, Self::S>>;

    /// given a `from` vertex, returns an iterator on (`to`, set of edges | optional edge)
    fn edges_by_to_iter(&self, &V) -> hash_map::Iter<V, Self::S>;

    /// given a `from` and a `to` vertex, returns a set of edges | optional edge
    fn get_edges(&self, &V, &V) -> Option<&Self::S>;
}
