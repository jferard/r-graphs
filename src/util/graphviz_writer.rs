/// *****************************************************************************
/// R-Graphs - A simple graph library for Rust
/// Copyright (C) 2016-2019 J. Férard <https://github.com/jferard>
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
use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::Path;
use util::graphviz_builder::GraphvizBuilder;

pub struct GraphvizWriter<'a, B>
    where B: 'a + GraphvizBuilder<'a>
{
    builder: &'a B,
}

impl<'a, H> GraphvizWriter<'a, H>
    where H: 'a + GraphvizBuilder<'a>
{
    pub fn new(h: &'a H) -> Self {
        GraphvizWriter { builder: h }
    }

    pub fn output(&self, filename: &str) {
        let s = self.builder.build_string();

        let path = Path::new(filename);
        match path.parent() {
            None => {}
            Some(parent_path) => {
                fs::create_dir_all(parent_path.to_str().expect("Expect an utf-8 path"))
                    .expect("Failed to create dir")
            }
        }

        File::create(filename)
            .expect(&("Error opening file: ".to_string() + filename))
            .write_all(s.as_bytes()) // utf-8 by default
            .ok()
            .expect("Writing graph to file failed");
    }
}