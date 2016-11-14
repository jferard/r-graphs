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

// Sedgewick, chapter 30
// CLRS, 21.1
pub trait RefDisjointSet<'a, T>
    where T: 'a
{
    fn new() -> Self;
    fn add_set(&mut self, x: &'a T) -> bool;
    fn find(&mut self, x: &'a T) -> Option<&'a T>;
    fn union(&mut self, x: &'a T, y: &'a T) -> bool;
}

pub trait ValueDisjointSet<T>
    where T: Copy
{
    fn new() -> Self;
    fn add_set(&mut self, x: T) -> bool;
    fn find(&mut self, x: T) -> Option<T>;
    fn union(&mut self, x: T, y: T) -> bool;
}

// trait DisjointSetVisitor<T> {
// 	fn find(&mut DisjointSet, x: &T) -> Option<&T>;
// 	fn union(&mut DisjointSet, x: &T, y: &T) -> bool;
// }
//
// impl DisjointSetVisitor<usize> for usize {
// 	fn find(&mut DisjointSet, x: &usize) -> Option<&usize> { return 0_usize; }
// 	fn union(&mut DisjointSet, x: &usize, y: &usize) -> { return true; }
// }
//
// impl DisjointSetVisitor<T> for T where T:Clone {
// 	fn find(&mut DisjointSet, x: &T) -> Option<&T> { return 0_usize; }
// 	fn union(&mut DisjointSet, x: &T, y: &T) -> { return true; }
// }
//
// struct S {}
//
// impl DisjointSet for S {
// 	fn find(&mut self, x: &T) -> Option<&T> { DisjointSetVisitor::find(self, x) };
// 	fn union(&mut self, x: &T, y: &T) -> bool;
// }
//
