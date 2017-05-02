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
use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::Path;

pub trait GraphvizHelper<'a, G>
    where G: 'a
{
    fn new(g: &'a G) -> Self;

    fn output(&self, filename: &str) {
        let s = self.build_string();

        let path = Path::new(filename);
        match path.parent() {
            None => {},
            Some(parent_path) => {
                fs::create_dir_all(parent_path.to_str().expect("expect an utf-8 path"))
                    .expect("failed to create dir") }
        }

        File::create(filename)
            .expect(&("Error opening file: ".to_string() + filename))
            .write_all(s.as_bytes()) // utf-8 by default
            .ok()
            .expect("Writing graph to file failed");
    }

    fn build_string(&self) -> String;

    fn mark(&mut self, vertices: Vec<usize>);
}

pub struct GraphvizHelperImpl<'a, G>
    where G: 'a // + Graph<'a>
{
    g: &'a G,
    marked_vertices: Vec<Vec<usize>>,
}

impl<'a, G> GraphvizHelperImpl<'a, G>
    where G: 'a
{
    // add color : grey for last marked, black for others
    fn add_color_to_subgraph<'b>(&self, s: &'b mut String, n: usize) {
        if n > 0 {
            if n > 1 {
                for m in 0..n - 1 {
                    for v in &self.marked_vertices[m] {
                        s.push_str(&format!("\t\"{0}_{1}\" [fontcolor=white, fillcolor=black, \
                                             style=filled]\n",
                                            n,
                                            v));
                    }
                }
            }
            for v in &self.marked_vertices[n - 1] {
                s.push_str(&format!("\t\"{0}_{1}\" [fillcolor=grey, style=filled]\n", n, v));
            }
        }
    }
}


include!("_graphviz_helper_directed.rs");
include!("_graphviz_helper_undirected.rs");
