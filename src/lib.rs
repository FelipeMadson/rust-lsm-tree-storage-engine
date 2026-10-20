use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub struct LsmEngine {
    memtable: Arc<RwLock<BTreeMap<String, String>>>,
    max_memtable_size: usize,
}

impl LsmEngine {
    pub fn new(max_memtable_size: usize) -> Self {
        Self {
            memtable: Arc::new(RwLock::new(BTreeMap::new())),
            max_memtable_size,
        }
    }

    pub fn put(&self, key: &str, value: &str) -> Result<(), &'static str> {
        let mut table = self.memtable.write().map_err(|_| "Lock poisoned")?;
        table.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let table = self.memtable.read().ok()?;
        table.get(key).cloned()
    }

    pub fn len(&self) -> usize {
        self.memtable.read().map(|t| t.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let engine = LsmEngine::new(100);
        assert!(engine.is_empty());

        engine.put("user:1001", "Felipe Madison").unwrap();
        engine.put("role:1001", "Principal Engineer").unwrap();

        assert_eq!(engine.get("user:1001"), Some("Felipe Madison".to_string()));
        assert_eq!(engine.get("role:1001"), Some("Principal Engineer".to_string()));
        assert_eq!(engine.get("non_existent"), None);
        assert_eq!(engine.len(), 2);
    }
}
