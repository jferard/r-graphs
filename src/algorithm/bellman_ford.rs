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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use algorithm::visitor::Visitor;
use graph::DecoratedGraph;
use graph::Graph;

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

pub struct BellmanFordBrowser<'a, G, V, V2>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          V2: 'a + Visitor
{
    decorated_graph: &'a G,
    source: usize,
    dist: Vec<Option<usize>>,
    previous: Vec<Option<usize>>,
    negative_cycle: bool,
    visitor: &'a mut V2,
    phantom_v: PhantomData<V>,
}

impl<'a, G, V, V2> BellmanFordBrowser<'a, G, V, V2>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          V2: 'a + Visitor
{
    pub fn new(decorated_graph: &'a G, source: usize, visitor: &'a mut V2) -> BellmanFordBrowser<'a, G, V, V2> {
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

    pub fn browse(&mut self) {
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
    }

    pub fn process(&mut self, dist_node: usize, node: usize) -> bool {
        let mut changed = false;
        self.visitor.visit(node, None);
        for neighbor in self.decorated_graph.adjacent_vertices_iter(node) {
            for (_, o_weight) in self.decorated_graph.edges_values_iter(node, neighbor) {
                let weight = match o_weight {
                    Some(w) => *w,
                    None => 0
                };
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

    pub fn path(&self, source: usize, dest: usize) -> Vec<usize> {
        let mut vec = Vec::new();
        vec.insert(0, dest);
        let mut i = self.previous[dest].expect("should have a previous node");
        while i != source {
            vec.insert(0, i);
            i = self.previous[i].expect("should have a previous node");
        }
        vec.insert(0, source);
        vec
    }

    pub fn has_negative_cycle(&self) -> bool {
        self.negative_cycle
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
    fn test_bellman_ford() {
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
                b.browse();
                path.push(b.path(source, dest));
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &marked_vertices);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/bellman.dot");
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &path);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/bellman2.dot");
            }
        }
    }
}
