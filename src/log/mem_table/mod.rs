use bytes::Bytes;
use vector_memtable::VectorMemTable;

use crate::config::{self, MemTableSettings};

pub mod vector_memtable;

pub struct MemTableEntry {
    pub key: Bytes,
    pub value: Bytes,
}

/// MemTableStore is a trait that defines the interface for the data structure that stores
/// the MemTable entries.
pub trait MemTableStore {
    fn put(&mut self, key: &[u8], value: &[u8]);
    fn delete(&mut self, key: &[u8]);
    fn get(&self, key: &[u8]) -> Option<Bytes>;
    fn approximate_size(&self) -> u128;
}

/// MemTable holds a sorted list of the latest written records.
///
/// Writes are duplicated to the WAL for recovery of the MemTable in the event of a restart.
///
/// MemTables have a max capacity and when that is reached, we flush the MemTable
/// to disk as a Table(SSTable).
///
/// Entries are stored in a datastructure that must be optimized for heavy insertion and fast
/// lookups
pub struct MemTable {
    /// Dyn dispatch so that we can construct and change the inner implementation of this mtfk at
    /// runtime
    store: Box<dyn MemTableStore>,
    /// Config for memtable general settings and it's inner implementation
    cfg: config::MemTableSettings,
}

impl MemTable {
    pub fn new(cfg: MemTableSettings) -> Self {
        let store: Box<dyn MemTableStore> = match cfg.table_impl {
            config::MemTableImplSettings::VectorMemTable(ref s) => {
                Box::new(VectorMemTable::new(s.clone()))
            }
        };

        MemTable { store, cfg }
    }
}
