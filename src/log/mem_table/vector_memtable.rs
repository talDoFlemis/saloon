use std::{mem, usize};

use bytes::Bytes;

use crate::config::VectorMemTableSettings;

use super::{MemTableEntry, MemTableStore};

pub struct VectorMemTable {
    entries: Vec<MemTableEntry>,
    size: u128,
    cfg: VectorMemTableSettings,
}

impl VectorMemTable {
    pub fn new(cfg: VectorMemTableSettings) -> Self {
        let initial_vec_size: usize = cfg
            .initial_vec_size
            .try_into()
            .expect("expected to initial vec size to be less than platform usize");

        VectorMemTable {
            entries: Vec::with_capacity(initial_vec_size),
            size: 0,
            cfg,
        }
    }

    fn get_index(&self, key: &[u8]) -> Result<usize, usize> {
        self.entries.binary_search_by_key(&key, |e| &e.key)
    }
}

impl MemTableStore for VectorMemTable {
    fn put(&mut self, key: &[u8], value: &[u8]) {
        let entry = MemTableEntry {
            key: Bytes::copy_from_slice(key),
            value: Bytes::copy_from_slice(value),
        };

        match self.get_index(key) {
            Ok(idx) => {
                let original_value_size = self.entries[idx].value.len();
                let new_value_size = value.len();
                let value_difference_size: u128 = (new_value_size - original_value_size)
                    .try_into()
                    .expect("expected the value difference to less than u128");
                self.size += value_difference_size;
                self.entries[idx] = entry;
            }
            // There is no key on the vector and we must add satellite data size too
            Err(idx) => {
                let new_entry_size: u128 = (key.len() + value.len())
                    .try_into()
                    .expect("expected new entry size to be less than u128");
                self.size += new_entry_size;
                self.entries.insert(idx, entry);
            }
        }
    }

    fn delete(&mut self, key: &[u8]) {
        if let Ok(idx) = self.get_index(key) {
            self.size -= self.entries[idx].value.len() as u128;
            self.entries[idx].value = Bytes::new();
        }
    }

    fn get(&self, key: &[u8]) -> Option<Bytes> {
        match self.get_index(key) {
            Ok(idx) => Some(self.entries[idx].value.clone()),
            Err(_) => None,
        }
    }

    fn approximate_size(&self) -> u128 {
        self.size
    }
}
