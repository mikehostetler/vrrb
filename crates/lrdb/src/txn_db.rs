use std::{collections::HashMap, sync::Arc, time::SystemTime};

use lr_trie::{InnerTrieWrapper, LeftRightTrie};
use patriecia::db::MemoryDB;
use primitives::TxHash;
use vrrb_core::txn::Txn;

use crate::result::{LeftRightDbError, Result};

#[derive(Debug, Clone)]
pub struct TxnDb<'a> {
    trie: LeftRightTrie<'a, TxHash, Txn, MemoryDB>,
    last_refresh: std::time::SystemTime,
}

impl<'a> Default for TxnDb<'a> {
    fn default() -> Self {
        let trie = LeftRightTrie::new(Arc::new(MemoryDB::new(true)));

        Self {
            // TODO: revisit to use utc time
            last_refresh: SystemTime::now(),
            trie,
        }
    }
}

impl<'a> TxnDb<'a> {
    /// Returns new, empty instance of TxnDb
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_handle(&self) -> TxnDbReadHandle {
        let inner = self.trie.handle();
        TxnDbReadHandle { inner }
    }
}

#[derive(Debug, Clone)]
pub struct TxnDbReadHandle {
    inner: InnerTrieWrapper<MemoryDB>,
}

impl TxnDbReadHandle {
    pub fn get(&self, key: &TxHash) -> Result<Txn> {
        self.inner
            .get(key)
            .map_err(|err| LeftRightDbError::Other(err.to_string()))
    }

    pub fn batch_get(&self, keys: Vec<TxHash>) -> HashMap<TxHash, Option<Txn>> {
        let mut accounts = HashMap::new();

        keys.iter().for_each(|key| {
            let value = self.get(key).ok();
            accounts.insert(key.to_owned(), value);
        });

        accounts
    }

    pub fn entries(&self) -> HashMap<TxHash, Txn> {
        // TODO: revisit and refactor into inner wrapper
        self.inner
            .iter()
            .map(|(key, value)| {
                let key = bincode::deserialize(&key).unwrap_or_default();
                let value = bincode::deserialize(&value).unwrap_or_default();

                (key, value)
            })
            .collect()
    }

    /// Returns a number of initialized accounts in the database
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns the information about the StateDb being empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
