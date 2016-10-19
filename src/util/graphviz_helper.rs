/*******************************************************************************
 * R-Graphs - A simple graph library for Rust
 * Copyright (C) 2016 J. Férard <https://github.com/jferard>
 *
 * This file is part of R-Graphs.
 *
 * R-Graphs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * R-Graphs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 ******************************************************************************/
use graph::graph::Graph;
use std::io::Write;
use std::fs::File;

pub trait GraphvizHelper<'a, G>
    where G: 'a + Graph<'a>
{
    fn new(g: &'a G) -> Self;
    fn output(&self, filename: &str) {
        let s = self.build_string();
        File::create(filename)
            .expect(&("Error opening file: ".to_string() + filename))
            .write_all(s.as_bytes())
            .ok()
            .expect("Writing graph to file failed");
    }

    fn build_string(&self) -> String;

    fn mark(&mut self, vertices: Vec<usize>);
}

pub struct GraphvizHelperImpl<'a, G>
    where G: 'a + Graph<'a>
{
    g: &'a G,
    marked_vertices: Vec<Vec<usize>>,
}

include!("_graphviz_helper_directed.rs");
include!("_graphviz_helper_undirected.rs");