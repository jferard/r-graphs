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
use std::collections::hash_map::Iter;
use std::iter;
use util::simple_edge_set::SimpleEdgeSet;
use graph::decorated_graph::DecoratedGraph;
use graph::graphs::UndirectedGraph;
use graph::graph::Graph;
use graph::graph_builder::GraphBuilder;
use graph::basic_graph::BasicGraph;

pub struct UndirectedSimpleGraphImpl {
    basic_graph: BasicGraph<SimpleEdgeSet<usize, usize>>,
    reversed: Vec<usize>,
}

impl<'a> GraphBuilder<'a> for UndirectedSimpleGraphImpl {
    type ES = SimpleEdgeSet<usize, usize>;

    fn new(basic_graph: BasicGraph<SimpleEdgeSet<usize, usize>>) -> UndirectedSimpleGraphImpl {
        UndirectedSimpleGraphImpl {
            reversed: Vec::with_capacity(basic_graph.edges_max()),
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
        let e1 = self.basic_graph.add_edge(v, u);
        let e2 = self.basic_graph.add_edge(u, v);
        self.reversed.insert(e1, e2);
        self.reversed.insert(e2, e1);
        e1
    }

    fn remove_edge(&mut self, e: usize) {
        let e2 = self.reversed[e];
        self.basic_graph.remove_edge(e);
        self.basic_graph.remove_edge(e2);
    }
}

impl<'a> Graph<'a> for UndirectedSimpleGraphImpl {
    type ElementIterator = Box<Iterator<Item=usize> + 'a>;
    type AdjacentVerticesIterator = Box<Iterator<Item=usize> + 'a>;
    type AdjacentVerticesAndEdgesIterator = Iter<'a, usize, usize>;

    fn get_edges_from_vertices_iter(&self, u: usize, v: usize) -> Box<Iterator<Item=usize>> {
        match self.basic_graph.get_edges_from_vertices(u, v) {
            None => {
                match self.basic_graph.get_edges_from_vertices(v, u) {
                    None => Box::new(iter::empty()),
                    Some(oe) => Box::new(iter::once(*oe)),
                }
            }
            Some(oe) => Box::new(iter::once(*oe)),
        }
    }

    fn get_vertices_from_edge(&self, e: usize) -> (usize, usize) {
        self.basic_graph.get_vertices_from_edge(e)
    }

    fn vertices_iter(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        self.basic_graph.vertices_iter()
    }

    fn edges_iter(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.basic_graph.edges_iter().filter(move |&e| self.is_main_edge(e)))
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

    fn adjacent_vertices_iter(&'a self, u: usize) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.basic_graph.direct_adjacent_vertices_iter(u).map(move |(&u, _)| u))
    }

    fn adjacent_vertices_and_edges_iter(&'a self, u: usize) -> Self::AdjacentVerticesAndEdgesIterator {
        self.basic_graph.direct_adjacent_vertices_iter(u) // chain
    }

    fn get_reversed_edge(&self, e: usize) -> usize {
        self.reversed[e]
    }
}

impl<'a> DecoratedGraph<'a, usize, usize> for UndirectedSimpleGraphImpl {
    fn vertices_value_iter(&'a self) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        Box::new(self.vertices_iter().map(move |i| (i, 1)))
    }

    fn edges_values_iter(&'a self, u: usize, v: usize) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        Box::new(self.get_edges_from_vertices_iter(u, v).map(move |e| (e, 1)))
    }
}

impl<'a> UndirectedGraph<'a> for UndirectedSimpleGraphImpl {}

impl<'a> UndirectedSimpleGraphImpl {
    fn is_main_edge(&self, e: usize) -> bool {
        let (u, v) = self.basic_graph.get_vertices_from_edge(e);
        u < v
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graph::Graph;
    use graph::basic_graph::BasicGraph;
    use util::GraphvizWriter;
    use util::GraphvizBuilder;
    use util::GraphvizBuilderUndirectedImpl;
    use graph::examples::graph1;

    #[test]
    fn test_gv() {
        let g: UndirectedSimpleGraphImpl = graph1();
        let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
        {
            let h = GraphvizBuilderUndirectedImpl::new(&g, &marked_vertices);
            let gw = GraphvizWriter::new(&h);
            gw.output("gv_output/graph1.dot");
        }
        marked_vertices.push(vec![1, 2]);
        marked_vertices.push(vec![5, 6]);
        {
            let h = GraphvizBuilderUndirectedImpl::new(&g, &marked_vertices);
            let gw = GraphvizWriter::new(&h);
            gw.output("gv_output/graph2.dot");
        }
    }

    #[test]
    fn test_adj() {
        let mut g = UndirectedSimpleGraphImpl::new(BasicGraph::new());
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
