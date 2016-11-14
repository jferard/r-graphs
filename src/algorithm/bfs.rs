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
use std::collections::VecDeque;
use graph::Graph;
use algorithm::visitor::Visitor;
use algorithm::visited::Visited;

pub struct BFSBrowser<'b, G, V>
    where G: 'b + Graph<'b>,
          V: 'b + Visitor
{
    g: &'b G,
    visitor: &'b mut V,
    visited: Visited,
    queue: VecDeque<(usize, Option<usize>)>,
}

impl<'b, G, V> BFSBrowser<'b, G, V>
    where G: 'b + Graph<'b>,
          V: 'b + Visitor
{
    pub fn new(g: &'b G, visitor: &'b mut V) -> BFSBrowser<'b, G, V> {
        BFSBrowser {
            g: g,
            visitor: visitor,
            visited: Visited::new(g.max()),
            queue: VecDeque::new(),
        }
    }

    pub fn browse(&mut self) {
        for u in self.g.vertices_iter() {
            if !self.visited.is_visited(u) {
                self.browse_from(u);
            }
        }
    }

    pub fn browse_from(&mut self, source: usize) {
        self.visited.set_visited(source);
        self.queue.push_back((source, None));

        while !self.queue.is_empty() {
            let (cur, parent) = self.queue.pop_front().unwrap();
            self.visitor.visit(cur, parent);
            match self.g.adjacent_vertices_iter(cur) {
                Some(m) => {
                    for (&u, _) in m {
                        if !self.visited.is_visited(u) {
                            self.visited.set_visited(u);
                            self.queue.push_back((u, Some(cur)));
                        }
                    }
                }
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util::GraphvizHelper;
    use util::GraphvizHelperImpl;
    use graph::DirectedSimpleGraphImpl;
    use graph::UndirectedSimpleGraphImpl;
    use graph::examples::graph2;

    #[test]
    fn test_bfs() {

        let g = graph2::<UndirectedSimpleGraphImpl>();
        {
            let mut gh: GraphvizHelperImpl<UndirectedSimpleGraphImpl> = GraphvizHelper::new(&g);
            {
                let mut b = BFSBrowser::new(&g, &mut gh);
                b.browse();
            }
            gh.output("gv_output/ubfs.dot");
        }
    }

    #[test]
    fn test_bfs2() {
        let g = graph2::<DirectedSimpleGraphImpl>();
        {
            let mut gh: GraphvizHelperImpl<DirectedSimpleGraphImpl> = GraphvizHelper::new(&g);
            {
                let mut b = BFSBrowser::new(&g, &mut gh);
                b.browse();
            }
            gh.output("gv_output/dbfs.dot");
        }
    }
}
