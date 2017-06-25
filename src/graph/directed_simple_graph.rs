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
use std::collections::hash_map;
use std::iter::Map;
use std::iter;

use util::simple_edge_set::SimpleEdgeSet;
use graph::Graph;
use graph::DirectedGraph;
use graph::GraphBuilder;
use graph::DecoratedGraph;
use graph::basic_graph::BasicGraph;

pub struct DirectedSimpleGraphImpl {
    basic_graph: BasicGraph<SimpleEdgeSet<usize, usize>>,
}

impl<'a> GraphBuilder<'a> for DirectedSimpleGraphImpl {
    type ES = SimpleEdgeSet<usize, usize>;

    fn new(basic_graph: BasicGraph<SimpleEdgeSet<usize, usize>>) -> DirectedSimpleGraphImpl {
        DirectedSimpleGraphImpl {
            basic_graph: basic_graph,
        }
    }

    fn create_vertex(&mut self) -> usize {
        self.basic_graph.add_vertex()
    }

    fn remove_vertex(&mut self, u: usize) {
        self.basic_graph.remove_vertex(u);
    }

    fn add_edge(&mut self, u: usize, v: usize) -> usize {
        self.basic_graph.add_edge(u, v)
    }

    fn remove_edge(&mut self, e: usize) {
        self.basic_graph.remove_edge(e);
    }
}

impl<'a> Graph<'a> for DirectedSimpleGraphImpl {
    type ElementIterator = Box<Iterator<Item=usize> + 'a>;
    type AdjacentVerticesIterator = Map<hash_map::Iter<'a, usize, usize>, fn((&usize, &usize)) -> usize>;
    type AdjacentEdgesByVertexIterator = hash_map::Iter<'a, usize, usize>;

    fn get_edges_from_vertices_iter(&self, u: usize, v: usize) -> Box<Iterator<Item=usize>> {
        match self.basic_graph.get_edges_from_vertices(u, v) {
            None => Box::new(iter::empty()),
            Some(oe) => Box::new(iter::once(*oe)),
        }
    }

    fn get_vertices_from_edge(&self, e: usize) -> Option<(usize, usize)> {
        self.basic_graph.get_vertices_from_edge(e)
    }

    fn vertices_iter(&'a self) -> Self::ElementIterator {
        self.basic_graph.vertices_iter()
    }

    fn edges_iter(&'a self) -> Self::ElementIterator {
        self.basic_graph.edges_iter()
    }

    fn vertices_size(&self) -> usize {
        self.basic_graph.vertices_size()
    }

    fn vertices_max(&self) -> usize {
        self.basic_graph.vertices_max()
    }

    fn edges_size(&self) -> usize {
        self.basic_graph.edges_size()
    }

    fn edges_max(&self) -> usize {
        self.basic_graph.edges_max()
    }

    fn adjacent_vertices_iter(&'a self, u: usize) -> Self::AdjacentVerticesIterator {
        self.basic_graph.direct_adjacent_vertices_iter(u).map(|(&u, _)| u)
    }
    fn adjacent_edges_by_vertex_iter(&'a self, u: usize) -> Self::AdjacentEdgesByVertexIterator {
        self.basic_graph.direct_adjacent_vertices_iter(u)
    }
    fn get_reversed_edge(&self, _: usize) -> Option<usize> {
        None
    }
}

impl<'a> DecoratedGraph<'a, usize, usize> for DirectedSimpleGraphImpl {
    fn vertices_value_iter(&'a self) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        Box::new(self.vertices_iter().map(move |i| (i, 1)))
    }

    fn edges_values_iter(&'a self, u: usize, v: usize) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        Box::new(self.get_edges_from_vertices_iter(u, v).map(move |e| (e, 1)))
    }
}

impl<'a> DirectedGraph<'a> for DirectedSimpleGraphImpl {}

#[cfg(test)]
mod test {
    use super::*;
    use graph::basic_graph::BasicGraph;

    #[test]
    fn test_adj() {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        for _ in 0..13 {
            g.create_vertex();
        }
        // ABCDEFG
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(0, 5);
        g.add_edge(0, 6);

        g.add_edge(3, 4);
        g.add_edge(3, 5);
        g.add_edge(4, 5);
    }
}
