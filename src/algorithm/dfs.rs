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
use algorithm::visitor::Visitor;
use algorithm::visited::Visited;

pub struct DFSBrowser<'b, G, V>
    where G: 'b + Graph<'b>,
          V: 'b + Visitor
{
    g: &'b G,
    visitor: &'b mut V,
    visited: Visited,
}

impl<'b, G, V> DFSBrowser<'b, G, V>
    where G: 'b + Graph<'b>,
          V: 'b + Visitor
{
    pub fn new(g: &'b G, visitor: &'b mut V) -> DFSBrowser<'b, G, V> {
        DFSBrowser {
            g: g,
            visitor: visitor,
            visited: Visited::new(g.max()),
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
        self.browse_from_helper(source, None);
    }

    fn browse_from_helper(&mut self, cur: usize, parent: Option<usize>) {
        self.visitor.visit(cur, parent);
        self.visited.set_visited(cur);
        let m = self.g.adjacent_vertices_iter(cur);
        for (&u, _) in m {
            if !self.visited.is_visited(u) {
                self.browse_from_helper(u, Some(cur));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util::GraphvizWriter;
    use util::GraphvizBuilderDirectedImpl;
    use util::GraphvizBuilder;
    use util::GraphvizBuilderUndirectedImpl;
    use graph::UndirectedSimpleGraphImpl;
    use graph::DirectedSimpleGraphImpl;
    use graph::examples::graph1;
    use graph::examples::graph2;

    #[test]
    fn test_dfs() {
        let g = graph1::<UndirectedSimpleGraphImpl>();
        {
            let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
            {
                let mut b = DFSBrowser::new(&g, &mut marked_vertices);
                b.browse();
            }
            let h = GraphvizBuilderUndirectedImpl::new(&g, &marked_vertices);
            let gw = GraphvizWriter::new(&h);
            gw.output("gv_output/udfs.dot");
        }
    }

    #[test]
    fn test_dfs2() {
        let g = graph2::<DirectedSimpleGraphImpl>();
        {
            let mut marked_vertices: Vec<Vec<usize>> = Vec::new();
            {
                let mut b = DFSBrowser::new(&g, &mut marked_vertices);
                b.browse();
            }
            let h = GraphvizBuilderDirectedImpl::new(&g, &marked_vertices);
            let gw = GraphvizWriter::new(&h);
            gw.output("gv_output/ddfs.dot");
        }
    }
}
