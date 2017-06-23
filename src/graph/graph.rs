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
use util::EdgeSet;
use graph::basic_graph::BasicGraph;

pub const VOID: usize = (-1_i8) as usize;

pub trait Graph<'a> {
    type ElementIterator: Iterator<Item = usize>;
    type AdjacentVerticesIterator: Iterator<Item = usize>;
    type AdjacentVerticesAndEdgesIterator: Iterator<Item = (&'a usize, &'a usize)>;

    /// return an iterator on edges. The iterator may be empty
    fn get_edges_from_vertices_iter(&self, usize, usize) -> Self::ElementIterator;

    /// return an Optional couple of vertices
    fn get_vertices_from_edge(&self, usize) -> Option<(usize, usize)>;

    /// return an Iterator on vertices
    fn vertices_iter(&'a self) -> Self::ElementIterator;

    /// return an Iterator on edges
    fn edges_iter(&'a self) -> Self::ElementIterator;

    /// return an Iterator on neighbors vertex
    fn adjacent_vertices_iter(&'a self, usize) -> Self::AdjacentVerticesIterator;

    /// return an Iterator on a (vertex, edge) collection
    fn adjacent_vertices_and_edges_iter(&'a self, usize) -> Self::AdjacentVerticesAndEdgesIterator;

    /// given an edge u->v, return the edge v->u, or VOID
    fn get_reversed_edge(&self, e: usize) -> usize;

    /// return the number of vertices
    fn size(&self) -> usize;

    /// return the maximum vertex
    fn max(&self) -> usize;
}
