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
pub mod graph;
pub mod graphs;
pub mod basic_graph;
pub mod examples;
pub mod undirected_simple_graph;
pub mod directed_simple_graph;
pub mod decorated_graph;
pub mod graph_decorator;
pub mod graph_builder;

pub use self::graph::Graph;
pub use self::undirected_simple_graph::UndirectedSimpleGraphImpl;
pub use self::directed_simple_graph::DirectedSimpleGraphImpl;
pub use self::decorated_graph::DecoratedGraph;
pub use self::graph_decorator::GraphDecorator;
