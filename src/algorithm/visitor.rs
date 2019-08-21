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
//use util::GraphvizBuilderDirectedImpl;
//use util::GraphvizHelperUndirectedImpl;
//use graph::UndirectedSimpleGraphImpl;
//use graph::DirectedSimpleGraphImpl;
//use util::graphviz_builder::GraphvizBuilder;

pub trait Visitor {
    #[allow(unused_variables)]
    fn visit(&mut self, node: usize, parent: Option<usize>) {}
}

/*
impl<'a> Visitor for GraphvizHelperUndirectedImpl {
    fn visit(&mut self, u: usize, _: Option<usize>) {
        self.mark(vec![u]);
    }
}

impl<'a> Visitor for GraphvizBuilderDirectedImpl {
    fn visit(&mut self, u: usize, _: Option<usize>) {
        self.mark(vec![u]);
    }
}*/

impl<'a> Visitor for Vec<Vec<usize>> {
    fn visit(&mut self, u: usize, _: Option<usize>) {
        self.push(vec![u]);
    }
}