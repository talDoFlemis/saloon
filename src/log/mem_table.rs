/// MemTableEntry is a struct that represents an entry in the MemTable.
pub struct MemTableEntry<K: Ord, V> {
    pub key: Vec<K>,
    pub value: Option<Vec<V>>,
    pub timestamp: u128,
    pub deleted: bool,
}

/// MemTableStore is a trait that defines the interface for the data structure that stores
/// the MemTable entries.
pub trait MemTableStore<K: Ord, V> {
    /// Set a key-value pair in the MemTable
    fn insert(&mut self, key: &K, value: V, timestamp: u128);
    /// Delete a key from the MemTable
    fn delete(&mut self, key: &K, timestamp: u128);
    /// Get a key from the MemTable
    fn get(&self, key: &K) -> Option<&MemTableEntry<K, V>>;
    /// Get the index of a key in the MemTable
    fn get_index(&self, key: &K) -> Result<usize, usize>;
    /// Get the length of the MemTable
    fn len(&self) -> u64;
    /// Get the entries of the MemTable
    fn entries(&self) -> &[MemTableEntry<K, V>];
    /// Get the size of the MemTable
    fn size(&self) -> u64;
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
pub struct MemTable<K, V, Store>
where
    K: Ord,
    Store: MemTableStore<K, V>,
{
    store: Store,
    size: u64,
    phantom_key: std::marker::PhantomData<K>,
    phantom_value: std::marker::PhantomData<V>,
}

impl<K, V, Store> MemTable<K, V, Store>
where
    K: Ord,
    Store: MemTableStore<K, V>,
{
    pub fn get(&self, key: &K) -> Option<&MemTableEntry<K, V>> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: &K, value: V, timestamp: u128) {
        self.store.insert(key, value, timestamp);
    }
}
