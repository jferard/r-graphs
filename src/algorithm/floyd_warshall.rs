use std::fmt::Debug;
use std::marker::PhantomData;

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

use graph::{DecoratedGraph, Graph};

pub struct FloydWarshallBrowser<'a, G, V>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug
{
    decorated_graph: &'a G,
    dist: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    v_count: usize,
    phantom_v: PhantomData<V>,
}

impl<'a, G, V> FloydWarshallBrowser<'a, G, V>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug
{
    pub fn new(decorated_graph: &'a G) -> FloydWarshallBrowser<'a, G, V> {
        let v_count = decorated_graph.vertices_max();

        FloydWarshallBrowser {
            decorated_graph,
            dist: vec![None; v_count * v_count],
            next: vec![None; v_count * v_count],
            v_count,
            phantom_v: PhantomData,
        }
    }

    pub fn browse(&mut self) {
        for e in self.decorated_graph.edges_iter() {
            let (u, v) = self.decorated_graph.get_vertices_from_edge(e).expect("");
            let w = self.decorated_graph.edges_values_iter(u, v).map(|(_, o_weight)| *o_weight.unwrap_or(&0)).min().expect("Should not happen");
            self.dist[u * self.v_count + v] = Some(w);
            self.next[u * self.v_count + v] = Some(v);
        }
        for u in self.decorated_graph.vertices_iter() {
            self.dist[u * self.v_count + u] = Some(0);
            self.next[u * self.v_count + u] = Some(u);
        }
        for t in self.decorated_graph.vertices_iter() {
            for u in self.decorated_graph.vertices_iter() {
                for v in self.decorated_graph.vertices_iter() {
                    match self.dist[u * self.v_count + v] {
                        None => {
                            match (self.dist[u * self.v_count + t], self.dist[t * self.v_count + v]) {
                                (Some(d1), Some(d2)) => {
                                    self.dist[u * self.v_count + v] = Some(d1 + d2);
                                    self.next[u * self.v_count + v] = self.next[u * self.v_count + t];
                                }
                                _ => {}
                            }
                        }
                        Some(d) => {
                            match (self.dist[u * self.v_count + t], self.dist[t * self.v_count + v]) {
                                (Some(d1), Some(d2)) => {
                                    if d1 + d2 < d {
                                        self.dist[u * self.v_count + v] = Some(d1 + d2);
                                        self.next[u * self.v_count + v] = self.next[u * self.v_count + t];
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn dist(&self, source: usize, target: usize) -> Option<usize> {
        self.dist[source * self.v_count + target]
    }

    pub fn path(&self, source: usize, target: usize) -> Vec<usize> {
        match self.next[source * self.v_count + target] {
            None => vec!(),
            _ => {
                let mut u = source;
                let mut path = vec!(u);
                while u != target {
                    u = self.next[u * self.v_count + target].expect("");
                    path.push(u);
                }
                path
            }
        }
    }
}

#[cfg(test)]
mod test {
    use graph::basic_graph::BasicGraph;
    use graph::DirectedSimpleGraphImpl;
    use graph::examples::decorated_graph1;
    use graph::GraphBuilder;
    use util::GraphvizBuilder;
    use util::GraphvizBuilderDirectedImpl;
    use util::GraphvizWriter;

    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        {
            let dg = decorated_graph1(&mut g);
            let mut path = Vec::new();
            {
                let mut b = FloydWarshallBrowser::new(&dg);
                b.browse();
                assert_eq!(vec!(0, 2, 3), b.path(0, 3));
                assert_eq!(vec!(0, 2, 3, 4, 5), b.path(0, 5));

                assert_eq!(Some(0), b.dist(0, 0));

                assert_eq!(Some(1), b.dist(0, 1));
                assert_eq!(Some(0), b.dist(1, 1));

                assert_eq!(Some(3), b.dist(0, 2));
                assert_eq!(None, b.dist(1, 2));
                assert_eq!(Some(0), b.dist(2, 2));

                assert_eq!(Some(4), b.dist(0, 3));
                assert_eq!(None, b.dist(1, 3));
                assert_eq!(Some(1), b.dist(2, 3));
                assert_eq!(Some(0), b.dist(3, 3));

                assert_eq!(Some(5), b.dist(0, 4));
                assert_eq!(None, b.dist(1, 4));
                assert_eq!(Some(2), b.dist(2, 4));
                assert_eq!(Some(1), b.dist(3, 4));
                assert_eq!(Some(0), b.dist(4, 4));

                assert_eq!(Some(12), b.dist(0, 5));
                assert_eq!(Some(12), b.dist(1, 5));
                assert_eq!(Some(9), b.dist(2, 5));
                assert_eq!(Some(8), b.dist(3, 5));
                assert_eq!(Some(7), b.dist(4, 5));
                assert_eq!(Some(0), b.dist(5, 5));

                assert_eq!(Some(9), b.dist(0, 6));
                assert_eq!(None, b.dist(1, 6));
                assert_eq!(Some(6), b.dist(2, 6));
                assert_eq!(Some(5), b.dist(3, 6));
                assert_eq!(Some(4), b.dist(4, 6));
                assert_eq!(None, b.dist(5, 6));
                assert_eq!(Some(0), b.dist(6, 6));

                path.push(b.path(0, 5));
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &path);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/floyd_path.dot");
            }
        }
    }
}
