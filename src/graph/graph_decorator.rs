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
use util::DenseVec;
use graph::Graph;
use graph::GraphBuilder;
use graph::DirectedGraph;
use graph::UndirectedGraph;
use graph::DecoratedGraph;
use std::fmt::Debug;

/// TODO
pub struct GraphDecorator<'a, G, V, E>
    where G: 'a + Graph<'a>,
          V: 'a + PartialEq + Clone + Debug,
          E: 'a + PartialEq + Clone + Debug
{
    graph: &'a mut G,
    vertex_decorations: DenseVec<'a, V>,
    edge_decorations: DenseVec<'a, E>,
}

impl<'a, G, V, E> GraphDecorator<'a, G, V, E>
    where G: Graph<'a> + GraphBuilder<'a>,
          V: 'a + PartialEq + Clone + Debug,
          E: 'a + PartialEq + Clone + Debug
{
    pub fn new(graph: &'a mut G) -> GraphDecorator<'a, G, V, E> {
        GraphDecorator {
            graph: graph,
            vertex_decorations: DenseVec::new(),
            edge_decorations: DenseVec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex_value: V) -> usize {
        let v = self.graph.create_vertex();
        self.vertex_decorations.add_value_at_place(v, vertex_value);
        v
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, edge_value: E) {
        let e1 = self.graph.add_edge(v1, v2);
        self.edge_decorations.add_value_at_place(e1, edge_value.clone());
        match self.graph.get_reversed_edge(e1) {
            None => {}
            Some(e2) => { self.edge_decorations.add_value_at_place(e2, edge_value); }
        }
    }
}

impl<'a, G, V, E> Graph<'a> for GraphDecorator<'a, G, V, E>
    where G: Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug {
    type VerticesIterator = G::VerticesIterator;
    type EdgesIterator = G::EdgesIterator;
    type EdgesFromVerticesIterator = G::EdgesFromVerticesIterator;
    type AdjacentVerticesIterator = G::AdjacentVerticesIterator;
    type AdjacentEdgesByVerticesIterator = G::AdjacentEdgesByVerticesIterator;

    fn vertices_iter(&'a self) -> Self::VerticesIterator {
        self.graph.vertices_iter()
    }

    fn adjacent_vertices_iter(&'a self, u: usize) -> Self::AdjacentVerticesIterator {
        self.graph.adjacent_vertices_iter(u) // chain
    }

    fn get_edges_from_vertices_iter(&self, u: usize, v: usize) -> Self::EdgesFromVerticesIterator {
        self.graph.get_edges_from_vertices_iter(u, v)
    }

    fn get_vertices_from_edge(&self, e: usize) -> Option<(usize, usize)> {
        self.graph.get_vertices_from_edge(e)
    }

    fn edges_iter(&'a self) -> Self::EdgesIterator {
        unimplemented!()
    }

    fn vertices_size(&self) -> usize {
        self.graph.vertices_size()
    }

    fn vertices_max(&self) -> usize {
        self.graph.vertices_max()
    }

    fn edges_size(&self) -> usize {
        self.graph.edges_size()
    }

    fn edges_max(&self) -> usize {
        self.graph.edges_max()
    }

    fn adjacent_edges_by_vertex_iter(&'a self, u: usize) -> Self::AdjacentEdgesByVerticesIterator {
        self.graph.adjacent_edges_by_vertex_iter(u)
    }

    fn get_reversed_edge(&self, e: usize) -> Option<usize> {
        self.graph.get_reversed_edge(e)
    }
}

impl<'a, G, V, E> DecoratedGraph<'a, &'a V, &'a E> for GraphDecorator<'a, G, V, E>
    where G: Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug {
    type VerticesValuesIterator = Box<Iterator<Item=(usize, &'a V)> + 'a>;
    type EdgesValuesIterator = Box<Iterator<Item=(usize, &'a E)> + 'a>;


    fn vertices_values_iter(&'a self) -> Self::VerticesValuesIterator {
        Box::new(self.graph.vertices_iter().map(move |i| (i, self.vertex_decorations.get_value(i).unwrap())))
    }

    fn edges_values_iter(&'a self, u: usize, v: usize) -> Self::EdgesValuesIterator {
        Box::new(self.graph.get_edges_from_vertices_iter(u, v).map(move |e| (e, self.edge_decorations.get_value(e).unwrap())))
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
    use graph::basic_graph::BasicGraph;
    use graph::undirected_simple_graph::UndirectedSimpleGraphImpl;
    use util::GraphvizBuilder;
    use util::GraphvizWriter;
    use util::GraphvizBuilderUndirectedImpl;
    use graph::examples::decorated_graph1;

    #[test]
    fn test_graphviz() {
        let mut g = UndirectedSimpleGraphImpl::new(BasicGraph::new());
        let dg = decorated_graph1(&mut g);
        {
            let v = Vec::new();
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
