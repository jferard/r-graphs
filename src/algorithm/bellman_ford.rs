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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use algorithm::visitor::Visitor;
use graph::DecoratedGraph;
use graph::Graph;
use algorithm::single_source_shortest_paths::SingleSourceShortestPathsImpl;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MinDistTo {
    min_dist: usize,
    to: usize,
}

impl Ord for MinDistTo {
    fn cmp(&self, other: &MinDistTo) -> Ordering {
        other.min_dist.cmp(&self.min_dist)
    }
}

impl PartialOrd for MinDistTo {
    fn partial_cmp(&self, other: &MinDistTo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BellmanFordBrowser<'a, G, V, W>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          W: 'a + Visitor
{
    decorated_graph: &'a G,
    source: usize,
    dist: Vec<Option<usize>>,
    previous: Vec<Option<usize>>,
    negative_cycle: bool,
    visitor: &'a mut W,
    phantom_v: PhantomData<V>,
}

impl<'a, G, V, W> BellmanFordBrowser<'a, G, V, W>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          W: 'a + Visitor
{
    pub fn new(decorated_graph: &'a G, source: usize, visitor: &'a mut W) -> BellmanFordBrowser<'a, G, V, W> {
        let mut dist = vec![None; decorated_graph.vertices_max()];
        dist[source] = Some(0);

        BellmanFordBrowser {
            decorated_graph,
            source,
            dist,
            previous: vec![None; decorated_graph.vertices_max()],
            negative_cycle: false,
            visitor,
            phantom_v: PhantomData,
        }
    }

    pub fn browse(&mut self) -> SingleSourceShortestPathsImpl {
        self.dist[self.source] = Some(0);
        for _ in 0..self.decorated_graph.edges_size() - 1 {
            for u in self.decorated_graph.vertices_iter() {
                if let Some(dist_u) = self.dist[u] {
                    self.process(dist_u, u);
                }
            }
        }
        for u in self.decorated_graph.vertices_iter() {
            if let Some(dist_u) = self.dist[u] {
                if self.process(dist_u, u) {
                    self.negative_cycle = true;
                    break;
                }
            }
        }
        SingleSourceShortestPathsImpl::new(self.source, &self.dist, &self.previous, self.negative_cycle)
    }

    fn process(&mut self, dist_node: usize, node: usize) -> bool {
        let mut changed = false;
        self.visitor.visit(node, None);
        for neighbor in self.decorated_graph.adjacent_vertices_iter(node) {
            for (_, o_weight) in self.decorated_graph.edges_values_iter(node, neighbor) {
                let weight = *o_weight.unwrap_or(&0);
                let dist_neighbor = dist_node + weight;
                if let Some(d) = self.dist[neighbor] {
                    if d <= dist_neighbor { // not interesting
                        continue;
                    }
                }
                self.dist[neighbor] = Some(dist_neighbor);
                self.previous[neighbor] = Some(node);
                changed = true;
            }
        }
        changed
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
    use algorithm::single_source_shortest_paths::SingleSourceShortestPaths;

    #[test]
    fn test_bellman_ford() {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        {
            let dg = decorated_graph1(&mut g);
            let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
            {
                let mut b = BellmanFordBrowser::new(&dg, 0, &mut marked_vertices);
                let x = b.browse();
                assert_eq!(Some(4), x.dist(3));
                assert_eq!(vec!(0, 2, 3), x.path(3));
                assert_eq!(Some(12), x.dist(5));
                assert_eq!(vec!(0, 2, 3, 4, 5), x.path(5));
            }
        }
    }

    #[test]
    fn graph_bellman_ford() {
        bellman_ford(0, 5);
    }

    fn bellman_ford(source: usize, dest: usize) {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        {
            let dg = decorated_graph1(&mut g);
            let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
            let mut path = Vec::new();
            {
                let mut b = BellmanFordBrowser::new(&dg, source, &mut marked_vertices);
                let x = b.browse();
                path.push(x.path(dest));
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &marked_vertices);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/bellman_visit.dot");
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &path);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/bellman_path.dot");
            }
        }
    }
}
