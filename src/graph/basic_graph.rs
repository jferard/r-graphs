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
use std::collections::hash_map::Iter as hm_Iter;
use util::dense_vec_indices::DenseVecIndices;
use util::edge_set::EdgeSet;

pub struct BasicGraph<E>
    where E: EdgeSet<usize, usize>
{
    vertices: DenseVecIndices,
    edges: DenseVecIndices,
    adjacent_vertices: E, // u => v | u -> v
    edge_to_vertices: HashMap<usize, (usize, usize)>, // e = (u, v)
}

/// Every vertex and every edge is identified by an index (usize).
impl<E> BasicGraph<E>
    where E: EdgeSet<usize, usize>
{
    pub fn new() -> BasicGraph<E> {
        BasicGraph {
            vertices: DenseVecIndices::new(),
            edges: DenseVecIndices::new(),

            adjacent_vertices: E::new(),

            edge_to_vertices: HashMap::new(),
        }
    }

    /// Get a free index for a vertex
    pub fn add_vertex(&mut self) -> usize {
        self.vertices.index_consume()
    }

    pub fn remove_vertex(&mut self, v: usize) {
        self.vertices.free_index(v);
    }

    /// Get a free index for an edge
    pub fn add_edge(&mut self, u: usize, v: usize) -> usize {
        let e = self.edges.index_consume();
        self.adjacent_vertices.add_edge(u, v, e);
        self.edge_to_vertices.insert(e, (u, v));
        e
    }

    pub fn remove_edge(&mut self, e: usize) {
        if self.edges.free_index(e) {
            self.edge_to_vertices.remove(&e);

            let (u, v) = self.get_vertices_from_edge(e).unwrap();
            self.adjacent_vertices.remove_edge(&u, &v, &e);
        }
    }

    pub fn get_edges_from_vertices(&self, u: usize, v: usize) -> Option<&E::S> {
        self.adjacent_vertices.get_edges(&u, &v)
    }

    pub fn get_vertices_from_edge(&self, e: usize) -> Option<(usize, usize)> {
        match self.edges.index_is_free(e) {
            true => None,
            false => Some(self.edge_to_vertices[&e]),

        }
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
        self.vertices.used_indices_iter()
    }

    pub fn edges_iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
        self.edges.used_indices_iter()
    }

    pub fn direct_adjacent_vertices_iter(&self, u: usize) -> Option<hm_Iter<usize, E::S>> {
        self.adjacent_vertices.edges_by_to_iter(&u)
    }

    pub fn size(&self) -> usize {
        self.vertices.size()
    }

    pub fn max(&self) -> usize {
        self.vertices.max()
    }
}
