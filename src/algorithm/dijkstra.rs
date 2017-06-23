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

use graph::Graph;
use graph::DecoratedGraph;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::cmp::Ordering;

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

pub struct DijkstraBrowser<'a, G, V>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
{
    decorated_graph: &'a G,
    heap: BinaryHeap<MinDistTo>,
    black: Vec<bool>,
    dist: Vec<usize>,
    prec: Vec<Option<usize>>,
    target: usize,
    phantomData: PhantomData<V>,
}

impl<'a, G, V> DijkstraBrowser<'a, G, V>
    where G: 'a + Graph<'a> + DecoratedGraph<'a, V, &'a usize>,
          V: 'a + PartialEq + Clone + Debug,
{
    pub fn new(decorated_graph: &'a G, source: usize, target: usize) -> DijkstraBrowser<'a, G, V> {
        let mut heap = BinaryHeap::new();
        heap.push(MinDistTo { min_dist:0, to:source} );
        DijkstraBrowser {
            decorated_graph: decorated_graph,
            heap: heap,
            black: vec![false; decorated_graph.max()],
            dist: vec![1000; decorated_graph.max()],
            prec: vec![None; decorated_graph.max()],
            target: target,
            phantomData: PhantomData,
        }
    }

    pub fn browse(&mut self) {
        loop {
            println!("{:?}", self.heap);
            println!("peek = {:?}", self.heap.peek());
            match self.heap.pop() {
                None => { println!("None"); break; }
                Some(MinDistTo { min_dist:_, to:node} ) if self.black[node] => { println!("black {:?}", node); }
                Some(MinDistTo { min_dist:_, to:node} ) if node == self.target => { println!("target {:?}", node); break; }
                Some(MinDistTo { min_dist:dist_cur_node, to:cur_node} ) => { self.process(dist_cur_node, cur_node); }
            }
        }
        println!("{:?}", self.prec);
    }

    pub fn process(&mut self, dist_node: usize, node: usize) {
        println!("{0}, {1}", dist_node, node);
        self.black[node] = true;
        for neighbor in self.decorated_graph.adjacent_vertices_iter(node) {
            println!("e= {0}", neighbor);
            for (_, weight) in self.decorated_graph.edges_values_iter(node, neighbor) {
                let dist_neighbor = dist_node + weight;
                if dist_neighbor < self.dist[neighbor] {
                    self.dist[neighbor] = dist_neighbor;
                    self.prec[neighbor] = Some(node);
                    self.heap.push(MinDistTo { min_dist:dist_neighbor, to:neighbor});
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use graph::Graph;
    use graph::DecoratedGraph;
    use graph::basic_graph::BasicGraph;
    use graph::DirectedSimpleGraphImpl;
    use graph::GraphBuilder;
    use graph::examples::decorated_graph1;

    #[test]
    fn test_dijkstra() {
        let mut g = DirectedSimpleGraphImpl::new(BasicGraph::new());
        let dg = decorated_graph1(&mut g);
        {
            let mut b = DijkstraBrowser::new(&dg, 0, 5);
            b.browse();
        }
    }
}
