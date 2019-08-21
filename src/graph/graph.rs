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
pub trait Graph<'a> {
    // let's define an associated type by iterator
    type VerticesIterator: Iterator<Item = usize>;
    type EdgesIterator: Iterator<Item = usize>;
    type EdgesFromVerticesIterator: Iterator<Item = usize>;
    type AdjacentVerticesIterator: Iterator<Item = usize>;
    type AdjacentEdgesByVerticesIterator: Iterator<Item = (&'a usize, &'a usize)>;

    /// return an Optional couple of vertices
    fn get_vertices_from_edge(&self, usize) -> Option<(usize, usize)>;

    /// given an edge u->v, return the edge v->u
    fn get_reversed_edge(&self, e: usize) -> Option<usize>;

    /// return the number of vertices
    fn vertices_size(&self) -> usize;

    /// return the maximum vertex
    fn vertices_max(&self) -> usize;

    /// return the number of edges
    fn edges_size(&self) -> usize;

    /// return the maximum edge
    fn edges_max(&self) -> usize;

    /// return an iterator on edges. The iterator may be empty
    fn get_edges_from_vertices_iter(&self, usize, usize) -> Self::EdgesFromVerticesIterator;

    /// return an Iterator on vertices
    fn vertices_iter(&'a self) -> Self::VerticesIterator;

    /// return an Iterator on edges
    fn edges_iter(&'a self) -> Self::EdgesIterator;

    /// return an Iterator on neighbors vertex
    fn adjacent_vertices_iter(&'a self, usize) -> Self::AdjacentVerticesIterator;

    /// return an Iterator on a (vertex, edge) collection
    fn adjacent_edges_by_vertex_iter(&'a self, usize) -> Self::AdjacentEdgesByVerticesIterator;

}
