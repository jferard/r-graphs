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
use std::iter::Map;
use std::collections::hash_map::Iter;
use util::dense_vec::DenseVec;
use util::edge_set::EdgeSet;
use graph::Graph;
use std::fmt::Debug;

pub trait DecoratedGraph<'a, V, E>: Graph<'a>
    where V: 'a + PartialEq + Clone + Debug,
          E: 'a + PartialEq + Clone + Debug
{
    fn vertices_value_iter(&'a self) -> Box<Iterator<Item=(usize, V)> + 'a>;
    fn edges_values_iter(&'a self, u: usize, v:usize) -> Box<Iterator<Item=(usize, E)> + 'a>;
}