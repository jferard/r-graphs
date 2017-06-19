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
use graph::graph::Graph;
use graph::graph_builder::GraphBuilder;
use graph::graphs::UndirectedGraph;
use graph::decorated_graph::DecoratedGraph;
use graph::graphs::DirectedGraph;
use std::marker::PhantomData;
use std::fmt::Debug;

/// TODO
pub struct GraphDecorator<'a, G, V, E>
    where G: 'a + Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug
{
    graph: &'a mut G,
    vertex_decorations: DenseVec<V>,
    edge_decorations: DenseVec<E>,
}

impl<'a, G, V, E> GraphDecorator<'a, G, V, E>
    where G: Graph<'a> + GraphBuilder<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug
{
    pub fn new(graph: &'a mut G) -> GraphDecorator<'a, G, V, E> {
        GraphDecorator {
            graph: graph,
            vertex_decorations: DenseVec::new(),
            edge_decorations: DenseVec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex_value: V) -> usize {
        let v = self.graph.add_vertex();
        self.vertex_decorations.add_value_at_place(v, vertex_value);
        v
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, edge_value: E) {
        let e = self.graph.add_edge(v1, v2);
        self.edge_decorations.add_value_at_place(e - 1, edge_value.clone());
        self.edge_decorations.add_value_at_place(e, edge_value);
    }
}

impl<'a, G, V, E> Graph<'a> for GraphDecorator<'a, G, V, E>
    where G: Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug {
    type ES = G::ES;
    type ElementIterator = G::ElementIterator;
    type AdjacentVerticesIterator = G::AdjacentVerticesIterator;

    fn vertices_iter(&'a self) -> G::ElementIterator {
        self.graph.vertices_iter()
    }

    fn adjacent_vertices_iter(&'a self, u: usize) -> G::AdjacentVerticesIterator {
        self.graph.adjacent_vertices_iter(u) // chain
    }

    fn get_edges_from_vertices(&self, _: usize, _: usize) -> Option<<Self::ES as EdgeSet<usize, usize>>::S> {
        unimplemented!()
    }

    fn get_edges_from_vertices_iter(&self, _: usize, _: usize) -> Self::ElementIterator {
        unimplemented!()
    }

    fn get_vertices_from_edge(&self, _: usize) -> Option<(usize, usize)> {
        unimplemented!()
    }

    fn edges_iter(&'a self) -> Self::ElementIterator {
        unimplemented!()
    }

    fn size(&self) -> usize {
        unimplemented!()
    }

    fn max(&self) -> usize {
        unimplemented!()
    }
}

impl<'a, G, V, E> DecoratedGraph<'a, V, E> for GraphDecorator<'a, G, V, E>
    where G: Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug {
    fn vertices_value_iter(&'a self) -> Box<Iterator<Item=(usize, V)> + 'a> {
        Box::new(self.graph.vertices_iter().map(move |i| (i, self.vertex_decorations.get_value(i).unwrap().clone())))
    }

    fn edges_values_iter(&'a self, u: usize, v: usize) -> Box<Iterator<Item=(usize, E)> + 'a> {
        Box::new(self.graph.get_edges_from_vertices_iter(u, v).map(move |e| (e, self.edge_decorations.get_value(e).unwrap().clone())))
    }
}

impl<'a, G, V, E> DirectedGraph<'a> for GraphDecorator<'a, G, V, E> where G: DirectedGraph<'a>,
                                                                          V: 'static + PartialEq + Clone + Debug,
                                                                          E: 'static + PartialEq + Clone + Debug
{}

impl<'a, G, V, E> UndirectedGraph<'a> for GraphDecorator<'a, G, V, E> where G: UndirectedGraph<'a>, V: 'static + PartialEq + Clone + Debug,
                                                                            E: 'static + PartialEq + Clone + Debug
{}

#[cfg(test)]
mod test {
    use super::*;
    use graph::Graph;
    use graph::decorated_graph::DecoratedGraph;
    use graph::basic_graph::BasicGraph;
    use graph::undirected_simple_graph::UndirectedSimpleGraphImpl;
    use util::GraphvizBuilder;
    use util::GraphvizWriter;
    use util::GraphvizBuilderUndirectedImpl;
    use graph::examples::decorated_graph1;

    #[test]
    fn testGV() {
        let mut g = UndirectedSimpleGraphImpl::new(BasicGraph::new());
        let dg = decorated_graph1(&mut g);
        {
            let mut v = Vec::new();
            let h = GraphvizBuilderUndirectedImpl::new(&dg, &v);
            let gw = GraphvizWriter::new(&h);
            gw.output("gv_output/graph3.dot");
        }


        // let mut gh = GraphvizHelperImpl::new(&g);
        // gh.output("gv_output/graph1.dot");
        // gh.mark(vec![1, 2]);
        // gh.mark(vec![5, 6]);
        // gh.output("gv_output/graph2.dot");
        //
    }
}
