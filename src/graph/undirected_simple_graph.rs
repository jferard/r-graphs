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
    g: BasicGraph<SimpleEdgeSet<usize, usize>>,
}

impl<'a> GraphBuilder<'a> for UndirectedSimpleGraphImpl {
    type ES = SimpleEdgeSet<usize, usize>;

    fn new(g1: BasicGraph<SimpleEdgeSet<usize, usize>>) -> UndirectedSimpleGraphImpl {
        UndirectedSimpleGraphImpl { g: g1 }
    }

    fn add_vertex(&mut self) -> usize {
        self.g.add_vertex()
    }

    fn remove_vertex(&mut self, u: usize) {
        self.g.remove_vertex(u);
    }

    fn add_edge(&mut self, u: usize, v: usize) -> usize {
        self.g.add_edge(v, u);
        self.g.add_edge(u, v)
    }

    fn remove_edge(&mut self, e: usize) {
        self.g.remove_edge(e);
        self.g.remove_edge(e - 1);
    }
}

impl<'a> Graph<'a> for UndirectedSimpleGraphImpl {
    type ES = SimpleEdgeSet<usize, usize>;
    type ElementIterator = Box<Iterator<Item=usize> + 'a>;
    type AdjacentVerticesIterator = Iter<'a, usize, usize>;

    fn get_edges_from_vertices(&self, u: usize, v: usize) -> Option<usize> {
        match self.g.get_edges_from_vertices(u, v) {
            None => {
                match self.g.get_edges_from_vertices(v, u) {
                    None => None,
                    Some(oe) => Some(*oe),
                }
            }
            Some(oe) => Some(*oe),
        }
    }

    fn get_edges_from_vertices_iter(&self, u: usize, v: usize) -> Box<Iterator<Item=usize>> {
        match self.g.get_edges_from_vertices(u, v) {
            None => {
                match self.g.get_edges_from_vertices(v, u) {
                    None => Box::new(iter::empty()),
                    Some(oe) => Box::new(iter::once(*oe)),
                }
            }
            Some(oe) => Box::new(iter::once(*oe)),
        }
    }

    fn get_vertices_from_edge(&self, e: usize) -> Option<(usize, usize)> {
        self.g.get_vertices_from_edge(e)
    }

    fn vertices_iter(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        self.g.vertices_iter()
    }

    fn edges_iter(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.g.edges_iter().filter(|&e| e % 2 == 1))
    }

    fn size(&self) -> usize {
        self.g.size()
    }

    fn max(&self) -> usize {
        self.g.max()
    }

    fn adjacent_vertices_iter(&'a self, u: usize) -> Iter<'a, usize, usize> {
        self.g.direct_adjacent_vertices_iter(u) // chain
    }
}

impl<'a> DecoratedGraph<'a, usize, usize> for UndirectedSimpleGraphImpl {
    fn vertices_value_iter(&'a self) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        Box::new(self.vertices_iter().map(move |i| (i, 1)))
    }

    fn edges_values_iter(&'a self, u: usize, v: usize) -> Box<Iterator<Item=(usize, usize)> + 'a> {
        match self.get_edges_from_vertices(u, v) {
            Some(s) => Box::new(iter::once((s, 1))),
            None => Box::new(iter::empty())
        }
    }
}

impl<'a> UndirectedGraph<'a> for UndirectedSimpleGraphImpl {}


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
            g.add_vertex();
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
