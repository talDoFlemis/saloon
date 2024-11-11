use std::mem;

use bytes::Bytes;

use crate::config::VectorMemTableSettings;

use super::{MemTableEntry, MemTableStore};

const SIZEOF_DELETED: usize = mem::size_of::<bool>();

pub struct VectorMemTable {
    entries: Vec<MemTableEntry>,
    size: u128,
    cfg: VectorMemTableSettings,
}

impl VectorMemTable {
    pub fn new(cfg: VectorMemTableSettings) -> Self {
        VectorMemTable {
            entries: Vec::with_capacity(cfg.initial_vec_size),
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
            deleted: false,
        };

        match self.get_index(key) {
            Ok(idx) => {
                let original_value_size = self.entries[idx].value.len();
                let new_value_size = value.len();
                let value_difference_size = new_value_size - original_value_size;
                self.size += value_difference_size as u128;
                self.entries[idx] = entry;
            }
            // There is no key on the vector and we must add satellite data size too
            Err(idx) => {
                self.size += (key.len() + value.len() + SIZEOF_DELETED) as u128;
                self.entries.insert(idx, entry);
            }
        }
    }

    fn delete(&mut self, key: &[u8]) {
        if let Ok(idx) = self.get_index(key) {
            self.size -= self.entries[idx].value.len() as u128;
            self.entries[idx].value = Bytes::new();
            self.entries[idx].deleted = true;
        }
    }

    fn get(&self, key: &[u8]) -> Option<&MemTableEntry> {
        match self.get_index(key) {
            Ok(idx) => Some(&self.entries[idx]),
            Err(_) => None,
        }
    }

    fn approximate_size(&self) -> u128 {
        self.size
    }
}
