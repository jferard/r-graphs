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
use graph::UndirectedSimpleGraphImpl;
use graph::Graph;
use util::graphviz_builder::GraphvizBuilder;
use util::graphviz_builder::Painter;

pub struct GraphvizBuilderUndirectedImpl<'a>
{
    marked_vertices: &'a Vec<Vec<usize>>,
    graph: &'a UndirectedSimpleGraphImpl,
    painter: Painter,
}

impl <'a> GraphvizBuilderUndirectedImpl<'a> {
    fn build_subgraph(&self, n: usize) -> String {
        let mut s = format!("subgraph cluster{0} {{\nlabel=\"Step {0}\"\n", n);
        for from in self.graph.vertices_iter() {
            s.push_str(&format!("\t\"{0}_{1}\" [label={1}]\n", n, from));
            match self.graph.adjacent_vertices_iter(from) {
                Some(m) => {
                    for to in m.filter_map(|(&u, _)| if u <= from { Some(u) } else { None }) {
                        s.push_str(&format!("\t\"{0}_{1}\" -- \"{0}_{2}\"\n", n, from, to));
                    }
                }
                _ => {}
            }
        }
        // add color : grey for last marked, black for others
        self.painter.add_color_to_subgraph(&mut s, n, self.marked_vertices);
        s.push_str("}\n");
        s
    }
}


impl<'a> GraphvizBuilder<'a> for
GraphvizBuilderUndirectedImpl<'a> {
    type G = UndirectedSimpleGraphImpl;

    fn new(graph: &'a Self::G, marked_vertices: &'a Vec<Vec<usize>>) -> GraphvizBuilderUndirectedImpl<'a> {
        GraphvizBuilderUndirectedImpl {
            marked_vertices: marked_vertices,
            graph: graph,
            painter: Painter::new()
        }
    }

    fn build_string(&self) -> String {
        let mut s = "graph".to_string();
        s.push_str(" G {\n");
        let l = self.marked_vertices.len();
        for n in 0..l + 1 {
            s.push_str(&self.build_subgraph(n));
        }
        s.push_str("}\n");
        s
    }
}
