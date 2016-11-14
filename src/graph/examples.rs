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
use graph::basic_graph::BasicGraph;
use graph::graph::Graph;

pub fn graph1<'a, T>() -> T
    where T: Graph<'a>
{
    let mut g: T = Graph::new(BasicGraph::new());
    for _ in 0..13 {
        g.add_vertex();
    }
    // ABCDEFG
    g.add_edge(0, 1);
    g.add_edge(2, 0);
    g.add_edge(0, 5);
    g.add_edge(0, 6);

    g.add_edge(3, 4);
    g.add_edge(5, 3);
    g.add_edge(4, 5);


    g.add_edge(4, 6);

    // HI
    g.add_edge(7, 8);

    // JKLM
    g.add_edge(9, 10);
    g.add_edge(11, 9);
    g.add_edge(9, 12);

    g.add_edge(11, 12);
    g
}

pub fn graph2<'a, T>() -> T
    where T: Graph<'a>
{
    let mut g: T = Graph::new(BasicGraph::new());
    for _ in 0..21 {
        g.add_vertex();
    }
    g.add_edge(0, 1);
    g.add_edge(2, 0);
    g.add_edge(0, 5);
    g.add_edge(0, 6);

    g.add_edge(3, 4);
    g.add_edge(5, 3);
    g.add_edge(4, 5);


    g.add_edge(4, 6);

    g.add_edge(7, 8);
    g.add_edge(13, 7);

    g.add_edge(9, 10);
    g.add_edge(11, 9);
    g.add_edge(9, 12);

    g.add_edge(11, 12);
    g.add_edge(13, 11);

    g.add_edge(1, 14);
    g.add_edge(14, 15);
    g.add_edge(14, 16);
    g.add_edge(16, 17);
    g.add_edge(16, 18);
    g.add_edge(17, 20);
    g.add_edge(18, 20);
    g.add_edge(18, 19);
    g.add_edge(18, 15);
    g.add_edge(15, 4);
    g
}
