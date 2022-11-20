use std::collections::HashMap;
use crate::annotation::Ann;

#[derive(Debug, Clone)]
pub struct AnnSet {
    pub items: HashMap<String, Ann>,
}

impl AnnSet {
    pub fn new() -> AnnSet {
        AnnSet { items: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> AnnSet {
        AnnSet { items: HashMap::with_capacity(capacity) }
    }
}

impl From<HashMap<String, Ann>> for AnnSet {
    fn from(hash_map: HashMap<String, Ann>) -> Self {
        AnnSet { items: hash_map }
    }
}

#[cfg(test)]
mod tests {
    use crate::annotationset::*;

    #[test]
    fn test() {
        let a = AnnSet::new();
        assert!(a.items.is_empty());
    }
}