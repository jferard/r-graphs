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
use graph::graph::Graph;

/// A Builder for a graphviz graph.
/// It builds a string from marked vertices and a given graph.
pub trait GraphvizBuilder<'a> {
    type G: Graph<'a>;

    fn new(graph: &'a Self::G, marked_vertices: &'a Vec<Vec<usize>>) -> Self;

    fn build_string(&self) -> String;
}

pub struct Painter{
}

impl Painter {
    pub fn new() -> Self {
        Painter { }
    }

    // add color : grey for last marked, black for others
    pub fn add_color_to_subgraph<'a>(&self, s: &mut String, n: usize, marked_vertices: &'a Vec<Vec<usize>>) {
        if n > 0 {
            if n > 1 {
                for m in 0..n - 1 {
                    for v in &marked_vertices[m] {
                        s.push_str(&format!("\t\"{0}_{1}\" [fontcolor=white, fillcolor=black, \
                                                 style=filled]\n",
                                            n,
                                            v));
                    }
                }
            }
            for v in &marked_vertices[n - 1] {
                s.push_str(&format!("\t\"{0}_{1}\" [fillcolor=grey, style=filled]\n", n, v));
            }
        }
    }
}