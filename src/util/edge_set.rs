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
use std::collections::hash_map::Iter;
use std::cmp::Eq;
use std::hash::Hash;

/// An EdgeSet is a set of edges.
/// One may add or remove an edge, and iterate on edges in various ways.
pub trait EdgeSet<V, E>
    where V: Eq + Hash,
          E: Eq + Hash
{
    type S; // we might use another iterator

    fn new() -> Self;
    fn add_edge(&mut self, V, V, E) -> bool;
    fn remove_edge(&mut self, &V, &V, &E) -> bool;

    fn edges_by_to_by_from_iter(&self) -> Iter<V, HashMap<V, Self::S>>;
    fn edges_by_to_iter(&self, &V) -> Option<Iter<V, Self::S>>;
    fn get_edges(&self, &V, &V) -> Option<&Self::S>;
}
