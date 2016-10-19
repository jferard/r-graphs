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
pub struct Visited
{
    visited: Vec<bool>,
}

impl Visited {
    pub fn new(size :usize) -> Visited {
         Visited {
         	visited: vec!(false ; size)
         }
    }

    pub fn is_visited(&mut self, u: usize) -> bool {
        if u > self.visited.len() + 1 {
            let missing = u - self.visited.len() + 1;
            self.visited.extend(vec!(false; missing));

        }
        self.visited[u]
    }

    pub fn set_visited(&mut self, u: usize) {
        if u > self.visited.len() + 1 {
            let missing = u - self.visited.len() + 1;
            self.visited.extend(vec!(false; missing));
        }
        self.visited[u] = true
    }
}