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
use graph::graph::Graph;
use util::edge_set::EdgeSet;

pub trait DirectedGraph<'a>: Graph<'a> {}

pub trait UndirectedGraph<'a>: Graph<'a> {}

pub trait SimpleGraph<'a>: Graph<'a> {}

pub trait MultiGraph<'a>: Graph<'a> {}

pub trait DirectedSimpleGraph<'a>: DirectedGraph<'a> + SimpleGraph<'a> {}

pub trait DirectedMultiGraph<'a>: DirectedGraph<'a> + MultiGraph<'a> {}

pub trait UndirectedSimpleGraph<'a>: UndirectedGraph<'a> + SimpleGraph<'a> {}

pub trait UndirectedMultiGraph<'a>: UndirectedGraph<'a> + MultiGraph<'a> {}
