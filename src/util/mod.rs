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

// disjoint set https://en.wikipedia.org/wiki/Disjoint-set_data_structure
pub mod disjoint_set;
pub mod ref_tree_disjoint_set;
pub mod usize_tree_disjoint_set;

pub mod graphviz_writer;
pub mod graphviz_painter;
pub mod graphviz_builder;
pub mod graphviz_builder_directed;
pub mod graphviz_builder_undirected;

// the sets of edges between couples of vertices
pub mod edge_set;
pub mod simple_edge_set;
pub mod multiple_edge_set;

// a compact associative table index -> value
pub mod dense_vec_indices;
mod dense_vec;
pub mod iterator_util;

pub use self::edge_set::EdgeSet;
pub use self::dense_vec::DenseVec;
pub use self::graphviz_writer::GraphvizWriter;
pub use self::graphviz_painter::GraphvizPainter;
pub use self::graphviz_builder::GraphvizBuilder;
pub use self::graphviz_builder_directed::GraphvizBuilderDirectedImpl;
pub use self::graphviz_builder_undirected::GraphvizBuilderUndirectedImpl;
