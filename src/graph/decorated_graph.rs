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
use std::marker::PhantomData;
use std::fmt::Debug;

/// TODO
pub struct DecoratedGraph<'a, G, V, E>
    where G: 'a + Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug
{
    graph: &'a mut G,
    vertex_decorations: DenseVec<V>,
    edge_decorations: DenseVec<E>,
}

impl<'a, G, V, E> DecoratedGraph<'a, G, V, E>
    where G: Graph<'a>,
          V: 'static + PartialEq + Clone + Debug,
          E: 'static + PartialEq + Clone + Debug
{
    pub fn new(graph: &'a mut G) -> DecoratedGraph<'a, G, V, E> {
        DecoratedGraph {
            graph: graph,
            vertex_decorations: DenseVec::new(),
            edge_decorations: DenseVec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex_value: V) -> usize {
        let v = self.graph.add_vertex();
        println!("{}", v);
        println!("v = {:?}", vertex_value);
        self.vertex_decorations.add_value_at_place(v, vertex_value);
        println!("{:?}", self.vertex_decorations.values_iter().collect::<Vec<V>>());
        v
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, edge_value: E) {
        let e = self.graph.add_edge(v1, v2);
        println!("{}", e);
        println!("e = {:?}", edge_value);
        self.edge_decorations.add_value_at_place((e-1)/2, edge_value);
        println!("{:?}", self.edge_decorations.values_iter().collect::<Vec<E>>());
    }

    pub fn vertices_iter(&'a self) -> G::ElementIterator {
        self.graph.vertices_iter()
    }

    pub fn vertices_label_iter(&'a self) -> Box<Iterator<Item=(usize, Option<&V>)> + 'a> {
        Box::new(self.graph.vertices_iter().map(move |i| (i, self.vertex_decorations.get_value(i))))
    }

    pub fn adjacent_vertices_iter(&'a self, u: usize) -> Option<G::AdjacentVerticesIterator> {
        self.graph.adjacent_vertices_iter(u) // chain
    }

    pub fn adjacent_vertices_label_iter(&'a self, u: usize) -> Option<G::AdjacentVerticesIterator> {
        self.graph.adjacent_vertices_iter(u) // chain
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graph::Graph;
    use graph::basic_graph::BasicGraph;
    use graph::undirected_simple_graph::UndirectedSimpleGraphImpl;
    use util::GraphvizBuilder;
    use util::GraphvizBuilderDecoratedUndirectedImpl;
    use graph::examples::decorated_graph1;

    #[test]
    fn testGV() {
        let mut g = UndirectedSimpleGraphImpl::new(BasicGraph::new());
        let dg = decorated_graph1(&mut g);
        let mut v = Vec::new();
        let gb = GraphvizBuilderDecoratedUndirectedImpl::new(&dg, &v);
        println!("{}", gb.build_string())
        // let mut gh = GraphvizHelperImpl::new(&g);
        // gh.output("gv_output/graph1.dot");
        // gh.mark(vec![1, 2]);
        // gh.mark(vec![5, 6]);
        // gh.output("gv_output/graph2.dot");
        //
    }
}
