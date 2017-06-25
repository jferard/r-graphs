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
use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::Hash;

pub struct HashMapHelper<K, V> {
    empty_hash_map: HashMap<K, V>,
    one_element_hash_map: HashMap<K, V>
}

impl<'a, K, V> HashMapHelper<K, V>
    where K: 'a + Eq + Hash,
          V: 'a
{
    pub fn new() -> HashMapHelper<K, V> {
        HashMapHelper {
            empty_hash_map: HashMap::new(),
            one_element_hash_map: HashMap::new(),
        }
    }

    pub fn empty(&'a self) -> hash_map::Iter<'a, K, V> {
        self.empty_hash_map.iter()
    }

    pub fn once(&'a mut self, k: K, v: V) -> hash_map::Iter<'a, K, V> {
        self.one_element_hash_map.clear();
        self.empty_hash_map.insert(k, v);
        self.empty_hash_map.iter()
    }
}
