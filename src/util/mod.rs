/*******************************************************************************
 * R-Graphs - A simple graph library for Rust
 * Copyright (C) 2016 J. Férard <https://github.com/jferard>
 *
 * This file is part of R-Graphs.
 *
 * R-Graphs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * R-Graphs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 ******************************************************************************/
pub mod disjoint_set;
pub mod ref_tree_disjoint_set;
pub mod usize_tree_disjoint_set;
pub mod dense_vec_indices;
pub mod dense_vec;
pub mod graphviz_helper;
pub mod edge_set;
pub mod simple_edge_set;
pub mod multiple_edge_set;

pub use self::graphviz_helper::GraphvizHelper;
pub use self::graphviz_helper::GraphvizHelperImpl;