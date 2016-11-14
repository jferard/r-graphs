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
use graph::dense_ref_vec::DenseRefVec;
use graph::graphs::Graph;

pub struct DecoratedGraph<G: Graph, V, E> {
    graph: G,
    vertex_decorations: DenseRefVec<V>,
    edge_decorations: DenseRefVec<E>,
}

impl<G: Graph, V, E> DecoratedGraph<G, V, E>
    where V: 'static + PartialEq + Clone,
          E: 'static + PartialEq + Clone
{
    fn new(graph: G) -> DecoratedGraph<G, V, E> {
        DecoratedGraph {
            graph: graph,
            vertex_decorations: DenseRefVec::new(),
            edge_decorations: DenseRefVec::new(),
        }
    }

    fn add_vertex(&mut self, vertex_value: V) -> usize {
        let v = self.graph.add_vertex();
        self.vertex_decorations.add_value_at_place(v, vertex_value);
        v
    }

    fn add_edge(&mut self, v1: usize, v2: usize, edge_value: E) {
        let e = self.graph.add_edge(v1, v2);
        self.edge_decorations.add_value_at_place(e, edge_value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graph::Graph;
    use graph::basic_graph::BasicGraph;
    use util::GraphvizHelper;
    use util::GraphvizHelperImpl;
    use graph::examples::graph1;

    #[test]
    fn testGV() {
        let g: UndirectedSimpleGraphImpl = graph1();
        let dg = DecoratedGraph::new(&g);
        // let mut gh = GraphvizHelperImpl::new(&g);
        // gh.output("gv_output/graph1.dot");
        // gh.mark(vec![1, 2]);
        // gh.mark(vec![5, 6]);
        // gh.output("gv_output/graph2.dot");
        //
    }
}
