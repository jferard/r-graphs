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
use util::edge_set::EdgeSet;
use graph::basic_graph::BasicGraph;

pub trait Graph<'a> {
    type ES: EdgeSet<usize, usize>;
    type ElementIterator: Iterator<Item = usize>;
    type AdjacentVerticesIterator: Iterator<Item = (&'a usize, &'a usize)>;

    fn new(e: BasicGraph<Self::ES>) -> Self;
    fn add_vertex(&mut self) -> usize;
    fn remove_vertex(&mut self, usize);
    fn add_edge(&mut self, usize, usize) -> usize;
    fn remove_edge(&mut self, usize);

    fn get_edge_from_vertices(&self, usize, usize) -> Option<usize>;
    fn get_vertices_from_edge(&self, usize) -> Option<(usize, usize)>;

    fn vertices_iter(&'a self) -> Self::ElementIterator;
    fn edges_iter(&'a self) -> Self::ElementIterator;

    fn adjacent_vertices_iter(&'a self, usize) -> Option<Self::AdjacentVerticesIterator>;

    fn size(&self) -> usize;
    fn max(&self) -> usize;
}
