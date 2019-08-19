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

use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::cmp::Ordering;

use graph::Graph;
use graph::VOID;
use graph::DecoratedGraph;
use algorithm::visitor::Visitor;

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

pub struct DijkstraBrowser<'a, G, V, V2>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          V2: 'a + Visitor
{
    decorated_graph: &'a G,
    heap: BinaryHeap<MinDistTo>,
    black: Vec<bool>,
    dist: Vec<usize>,
    prec: Vec<usize>,
    target: usize,
    visitor: &'a mut V2,
    phantom_v: PhantomData<V>,
}

impl<'a, G, V, V2> DijkstraBrowser<'a, G, V, V2>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
          V2: 'a + Visitor
{
    pub fn new(decorated_graph: &'a G, source: usize, target: usize, visitor: &'a mut V2) -> DijkstraBrowser<'a, G, V, V2> {
        let mut heap = BinaryHeap::new();
        heap.push(MinDistTo { min_dist: 0, to: source });
        DijkstraBrowser {
            decorated_graph,
            heap,
            black: vec![false; decorated_graph.vertices_max()],
            dist: vec![VOID; decorated_graph.vertices_max()],
            prec: vec![VOID; decorated_graph.vertices_max()],
            target,
            visitor,
            phantom_v: PhantomData,
        }
    }

    pub fn browse(&mut self) {
        loop {
            match self.heap.pop() {
                None => {
                    break;
                }
                Some(MinDistTo { min_dist: _, to: node }) if self.black[node] => {}
                Some(MinDistTo { min_dist: _, to: node }) if node == self.target => {
                    self.visitor.visit(node, None);
                    break;
                }
                Some(MinDistTo { min_dist: dist_cur_node, to: cur_node }) => { self.process(dist_cur_node, cur_node); }
            }
        }
    }

    pub fn process(&mut self, dist_node: usize, node: usize) {
        self.visitor.visit(node, None);
        self.black[node] = true;
        for neighbor in self.decorated_graph.adjacent_vertices_iter(node) {
            for (_, oweight) in self.decorated_graph.edges_values_iter(node, neighbor) {
                let weight = match oweight {
                    Some(w) => *w,
                    None => 0
                };
                let dist_neighbor = dist_node + weight;
                if dist_neighbor < self.dist[neighbor] {
                    self.dist[neighbor] = dist_neighbor;
                    self.prec[neighbor] = node;
                    self.heap.push(MinDistTo { min_dist: dist_neighbor, to: neighbor });
                }
            }
        }
    }

    pub fn path(&self, source: usize, dest: usize) -> Vec<usize> {
        let mut vec = Vec::new();
        vec.insert(0, dest);
        let mut i = self.prec[dest];
        while i != source {
            vec.insert(0, i);
            i = self.prec[i];
        }
        vec.insert(0, source);
        vec
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graph::basic_graph::BasicGraph;
    use graph::DirectedSimpleGraphImpl;
    use graph::GraphBuilder;
    use graph::examples::decorated_graph1;
    use util::GraphvizBuilderDirectedImpl;
    use util::GraphvizWriter;
    use util::GraphvizBuilder;


    #[test]
    fn test_dijkstra() {
        dijkstra(0, 5);
    }

    fn dijkstra(source: usize, dest: usize) {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        {
            let dg = decorated_graph1(&mut g);
            let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
            let mut path = Vec::new();
            {
                let mut b = DijkstraBrowser::new(&dg, source, dest, &mut marked_vertices);
                b.browse();
                path.push(b.path(source, dest));
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &marked_vertices);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/dijsktra.dot");
            }
            {
                let h = GraphvizBuilderDirectedImpl::new(&dg, &path);
                let gw = GraphvizWriter::new(&h);
                gw.output("gv_output/dijsktra2.dot");
            }
        }
    }
}
