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

pub trait SingleSourceShortestPaths {
    fn dist(&self, target: usize) -> Option<usize>;

    fn path(&self, target: usize) -> Vec<usize>;

    fn has_negative_cycle(&self) -> bool;
}

pub struct SingleSourceShortestPathsImpl<'a> {
    source: usize,
    dist: &'a Vec<Option<usize>>,
    previous: &'a Vec<Option<usize>>,
    negative_cycle: bool,
}

impl<'a> SingleSourceShortestPathsImpl<'a> {
    pub(crate) fn new(source: usize, dist: &'a Vec<Option<usize>>, previous: &'a Vec<Option<usize>>, negative_cycle: bool) -> Self {
        SingleSourceShortestPathsImpl {
            source,
            dist,
            previous,
            negative_cycle,
        }
    }
}

impl<'a> SingleSourceShortestPaths for SingleSourceShortestPathsImpl<'a> {
    fn dist(&self, target: usize) -> Option<usize> {
        self.dist[target]
    }

    fn path(&self, target: usize) -> Vec<usize> {
        let mut vec = Vec::new();
        vec.insert(0, target);
        let mut u = self.previous[target].expect("should have a previous node");
        while u != self.source {
            vec.insert(0, u);
            u = self.previous[u].expect("should have a previous node");
        }
        vec.insert(0, self.source);
        vec
    }

    fn has_negative_cycle(&self) -> bool {
        self.negative_cycle
    }
}