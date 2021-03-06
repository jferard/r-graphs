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
use graph::Graph;
use std::fmt::Debug;

pub trait DecoratedGraph<'a, V, E>: Graph<'a>
    where V: 'a + PartialEq + Clone + Debug,
          E: 'a + PartialEq + Clone + Debug
{
    type VerticesValuesIterator: Iterator<Item=(usize, Option<V>)>;
    type EdgesValuesIterator: Iterator<Item=(usize, Option<E>)>;

    fn vertices_values_iter(&'a self) -> Self::VerticesValuesIterator;
    fn edges_values_iter(&'a self, u: usize, v: usize) -> Self::EdgesValuesIterator;
}