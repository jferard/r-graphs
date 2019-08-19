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
use std::collections::VecDeque;
use std::iter::FilterMap;
use std::iter::Enumerate;
use std::slice::Iter;

/// DenseVecIndices represents a subset of natural numbers.
/// One uses `index_consume` to get the next index. This index is either an index that has been
/// freed (see `free_index`) or an new index.
pub struct DenseVecIndices {
    is_free: Vec<bool>,
    free_elements: VecDeque<usize>,
}

pub type UsedIndicesIter<'a> = FilterMap<Enumerate<Iter<'a, bool>>, fn((usize, &bool)) -> Option<usize>>;

impl DenseVecIndices {
    pub fn new() -> DenseVecIndices {
        DenseVecIndices {
            is_free: Vec::new(),
            free_elements: VecDeque::new(),
        }
    }

    pub fn with_capacity(n: usize) -> DenseVecIndices {
        DenseVecIndices {
            is_free: Vec::with_capacity(n),
            free_elements: VecDeque::new(),
        }
    }

    pub fn new_dense(n: usize) -> DenseVecIndices {
        DenseVecIndices {
            is_free: vec!(false; n),
            free_elements: VecDeque::new(),
        }
    }

    /// Return a new free index.
    pub fn index_consume(&mut self) -> usize {
        match self.free_elements.pop_front() {
            None => {
                self.is_free.push(false);
                self.is_free.len() - 1
            }
            Some(e) => {
                self.is_free[e] = false;
                e
            }
        }
    }

    /// Free an index.
    pub fn free_index(&mut self, e: usize) -> bool {
        if self.index_is_free(e) {
            return false;
        }

        if e == self.is_free.len() - 1 {
            self.is_free.pop();
        } else {
            self.is_free[e] = true;
            self.free_elements.push_front(e);
        }
        return true;
    }

    /// Return an iterator on all non free indices.
    /// heap cost : use into_iter
    pub fn used_indices_iter(&self) -> UsedIndicesIter {
        self.is_free.iter().enumerate().filter_map(|(e, &free)| match free {
            true => None,
            false => Some(e),
        })
    }

    pub fn index_is_free(&self, e: usize) -> bool {
        e >= self.is_free.len() || self.is_free[e]
    }

    pub fn index_is_used(&self, e: usize) -> bool {
        e < self.is_free.len() && !self.is_free[e]
    }

    pub fn size(&self) -> usize {
        self.is_free.len() - self.free_elements.len()
    }

    pub fn max(&self) -> usize {
        self.is_free.len()
    }

    pub fn capacity(&self) -> usize {
        self.is_free.capacity() + self.free_elements.len() - self.is_free.len()
    }
}

/// Return an iterator on the used indices.
impl<'a> IntoIterator for &'a DenseVecIndices {
    type Item = usize;
    type IntoIter = FilterMap<Enumerate<Iter<'a, bool>>, fn((usize, &'a bool)) -> Option<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        fn f((e, &free): (usize, &bool)) -> Option<usize> {
            match free {
                true => None,
                false => Some(e),
            }
        }
        self.is_free.iter().enumerate().filter_map(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dense_vec_indices1() {
        let mut set = DenseVecIndices::new();
        assert_eq!(set.size(), 0);
        assert!(set.index_is_free(0));
        assert_eq!(set.index_consume(), 0);
        assert_eq!(set.size(), 1);
        assert!(set.index_is_used(0));
        assert!(set.index_is_free(1));
    }

    #[test]
    fn test_dense_vec_indices2() {
        let mut set = DenseVecIndices::new();
        assert_eq!(set.index_consume(), 0);
        assert_eq!(set.index_consume(), 1);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.index_consume(), 3);
        assert_eq!(set.size(), 4);
    }

    #[test]
    fn test_dense_vec_indices3() {
        let mut set = DenseVecIndices::new();
        assert_eq!(set.index_consume(), 0);
        assert_eq!(set.index_consume(), 1);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.index_consume(), 3);
        set.free_index(2);
        assert_eq!(set.size(), 3);
        let v: Vec<usize> = set.used_indices_iter().collect();
        assert!(v == vec![0, 1, 3]);
    }

    #[test]
    fn test_dense_vec_indices4() {
        let mut set = DenseVecIndices::new();
        assert_eq!(set.index_consume(), 0);
        assert_eq!(set.index_consume(), 1);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.index_consume(), 3);
        set.free_index(2);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.size(), 4);
        let v: Vec<usize> = set.used_indices_iter().collect();
        assert!(v == vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dense_vec_indices5into_iter() {
        let mut set = DenseVecIndices::new();
        assert_eq!(set.index_consume(), 0);
        assert_eq!(set.index_consume(), 1);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.index_consume(), 3);
        set.free_index(2);
        assert_eq!(set.index_consume(), 2);
        assert_eq!(set.index_consume(), 4);
        assert_eq!(set.size(), 5);
        let v: Vec<usize> = set.into_iter().collect();
        assert!(v == vec![0, 1, 2, 3, 4]);
    }
}
