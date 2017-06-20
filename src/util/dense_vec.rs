/// *****************************************************************************
/// R-Graphs - A simple graph library for Rust
/// Copyright (C) 2016-2017 J. Férard <https://github.com/jferard>
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
use std::marker::PhantomData;
use util::dense_vec_indices::DenseVecIndices;

/// A DenseVec is a Vec with some holes. It is an associative table between a subset of
/// natural numbers represented by a DenseVecIndices and T values.
pub struct DenseVec<'a, T> where T: 'a {
    indices: DenseVecIndices,
    values: Vec<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> DenseVec<'a, T> where T: 'a + Clone + PartialEq {
    pub fn new() -> DenseVec<'a, T> {
        DenseVec {
            indices: DenseVecIndices::new(),
            values: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Get a free place and add a value.
    /// Return the index of the free place.
    pub fn add_value(&mut self, value: T) -> usize {
        let i = self.indices.index_consume();
        self.values[i] = value;
        i
    }

    /// Add a value at a given place. The place must be the next returned by index_consume
    pub fn add_value_at_place(&mut self, element: usize, value: T) {
        let e = self.indices.index_consume();
//        println!("{} != {}", e, element);
        assert! (e == element);
        if e == self.values.len() {
            self.values.push(value);
        } else {
            self.values[e] = value;
        }
    }

    /// Free the place
    pub fn remove_element(&mut self, e: usize) {
        self.indices.free_index(e);
    }

    /// Return an iterator on values
    /// heap cost : use into_iter
    pub fn values_iter<'b>(&'b self) -> Box<Iterator<Item=T> + 'b> {
        let it = self.indices.used_indices_iter();
        Box::new(it.map(move |e| self.values[e].clone()))
    }

    /// Gert the value at the index e
    pub fn get_value(&self, e: usize) -> Option<&T> {
        match self.indices.index_is_free(e) {
            true => None,
            false => Some(&self.values[e]),
        }
    }

    pub fn get_mut_value(&mut self, e: usize) -> Option<&mut T> {
        match self.indices.index_is_free(e) {
            false => None,
            true => Some(&mut self.values[e]),
        }
    }

    pub fn has_element(&self, e: usize) -> bool {
        self.indices.index_is_used(e)
    }

    pub fn has_value(&self, e: usize, v: &T) -> bool {
        self.indices.index_is_free(e) && self.values[e] == *v
    }

    pub fn size(&self) -> usize {
        self.indices.size()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dense_ref_vec1() {
        let mut set: DenseVec<usize> = DenseVec::new();
        assert! (set.size() == 0);
        assert! (!set.has_element(0));
        set.add_value_at_place(0, 10);
        assert!(set.size() == 1);
        assert! (set.has_element(0));
        assert! (set.has_element(0));
        assert! ( !set.has_element(1));
    }

    #[test]
    fn test_dense_ref_vec2() {
        let mut set: DenseVec<usize> = DenseVec::new();
        set.add_value_at_place(0, 10);
        set.add_value_at_place(1, 20);
        set.add_value_at_place(2, 30);
        set.add_value_at_place(3, 40);
        assert! (set.size() == 4);
    }

    #[test]
    fn test_dense_vec3() {
        let mut set: DenseVec<usize> = DenseVec::new();
        set.add_value_at_place(0, 10);
        set.add_value_at_place(1, 20);
        set.add_value_at_place(2, 30);
        set.add_value_at_place(3, 40);
        set.remove_element(2);
        assert!(set.size() == 3);
        let v: Vec<usize> = set.values_iter().collect();
        assert!(v == vec![10, 20, 40]);
    }

    #[test]
    fn test_dense_vec4() {
        let mut set: DenseVec<usize> = DenseVec::new();
        set.add_value_at_place(0, 10);
        set.add_value_at_place(1, 20);
        set.add_value_at_place(2, 30);
        set.add_value_at_place(3, 40);
        set.remove_element(2);
        set.add_value_at_place(2, 50);
        assert! (set.size() == 4);
        let v: Vec<usize> = set.values_iter().collect();
        assert! (v == vec![10, 20, 50, 40]);
    }
}
