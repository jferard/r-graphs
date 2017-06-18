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
use std::fmt::Display;
use graph::UndirectedSimpleGraphImpl;
use graph::Graph;
use graph::DecoratedGraph;
use util::graphviz_builder::GraphvizBuilder;
use util::graphviz_builder::Painter;
use std::fmt::Debug;

pub struct GraphvizBuilderDecoratedUndirectedImpl<'a, V, E>
    where V: 'static + PartialEq + Clone + Debug + Display,
          E: 'static + PartialEq + Clone + Debug + Display
{
    marked_vertices: &'a Vec<Vec<usize>>,
    graph: &'a DecoratedGraph<'a, UndirectedSimpleGraphImpl, V, E>,
    painter: Painter,
}

impl<'a, V, E> GraphvizBuilderDecoratedUndirectedImpl<'a, V, E>
    where V: 'static + PartialEq + Clone + Debug + Display,
          E: 'static + PartialEq + Clone + Debug + Display
{
    fn build_subgraph(&self, n: usize) -> String {
        let mut s = format!("subgraph cluster{0} {{\nlabel=\"Step {0}\"\n", n);
        for (from, label) in self.graph.vertices_label_iter() {
            println!("{}", from);
            println!("{}", label.unwrap());
            s.push_str(&format!("\t\"{0}_{1}\" [label={2}]\n", n, from, label.unwrap()));
            match self.graph.adjacent_vertices_label_iter(from) {
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

    pub fn new(graph: &'a DecoratedGraph<'a, UndirectedSimpleGraphImpl, V, E>, marked_vertices: &'a Vec<Vec<usize>>) -> GraphvizBuilderDecoratedUndirectedImpl<'a, V, E> {
        GraphvizBuilderDecoratedUndirectedImpl {
            marked_vertices: marked_vertices,
            graph: graph,
            painter: Painter::new()
        }
    }

    pub fn build_string(&self) -> String {
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
