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
use util::edge_set::EdgeSet;
use graph::basic_graph::BasicGraph;

pub trait GraphBuilder<'a> {
    type ES: EdgeSet<usize, usize>;

    /// create a new GraphBuilder
    fn new(e: BasicGraph<Self::ES>) -> Self;

    /// create a vertex
    fn create_vertex(&mut self) -> usize;

    /// remove a vertex
    fn remove_vertex(&mut self, u: usize);

    /// add an edge between u and v, and return a number
    fn add_edge(&mut self, u: usize, v: usize) -> usize;

    /// remove an edge
    fn remove_edge(&mut self, e: usize);
}
