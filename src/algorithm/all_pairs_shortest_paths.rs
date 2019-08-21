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

pub trait AllPairsShortestPaths {
    fn dist(&self, source: usize, target: usize) -> Option<usize>;

    fn path(&self, source: usize, target: usize) -> Vec<usize>;

    fn has_negative_cycle(&self) -> bool;
}

pub struct AllPairsShortestPathsImpl<'a> {
    v_count: usize,
    dist: &'a Vec<Option<usize>>,
    next: &'a Vec<Option<usize>>,
    negative_cycle: bool,
}

impl<'a> AllPairsShortestPathsImpl<'a> {
    pub(crate) fn new(v_count: usize, dist: &'a Vec<Option<usize>>, next: &'a Vec<Option<usize>>, negative_cycle: bool) -> Self {
        AllPairsShortestPathsImpl {
            v_count,
            dist,
            next,
            negative_cycle,
        }
    }
}

impl<'a> AllPairsShortestPaths for AllPairsShortestPathsImpl<'a> {
    fn dist(&self, source: usize, target: usize) -> Option<usize> {
        self.dist[source * self.v_count + target]
    }

    fn path(&self, source: usize, target: usize) -> Vec<usize> {
        match self.next[source * self.v_count + target] {
            None => vec!(),
            _ => {
                let mut u = source;
                let mut path = vec!(u);
                while u != target {
                    u = self.next[u * self.v_count + target].expect("");
                    path.push(u);
                }
                path
            }
        }
    }

    fn has_negative_cycle(&self) -> bool {
        self.negative_cycle
    }
}