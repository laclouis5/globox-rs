use std::{
    collections::{HashMap, hash_map::{IntoValues, Values}},
};
use crate::annotation::Ann;

/// A set of annotations with efficient indexing by image id.
/// 
/// Annotations cannot be modified in-place. Modification is
/// achived through a remove-insert operation or by consuming
/// and mapping.
#[derive(Debug, Clone)]
pub struct AnnSet {
    /// Warning: do not mutate the annotation labels.
    pub(crate) items: HashMap<String, Ann>,
}

impl AnnSet {
    pub fn new() -> AnnSet {
        AnnSet { items: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> AnnSet {
        AnnSet { items: HashMap::with_capacity(capacity) }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional)
    }
}

impl AnnSet {
    // TODO: Avoid cloning for the key.
    pub fn insert(&mut self, ann: Ann) -> Option<Ann> {
        self.items.insert(ann.img_id.clone(), ann)
    }

    pub fn get(&self, img_id: &str) -> Option<&Ann> {
        self.items.get(img_id)
    }

    /// Warning: do not mutate the annotation label as it
    /// breaks the invariant.
    pub(crate) fn get_mut(&mut self, img_id: &str) -> Option<&mut Ann> {
        self.items.get_mut(img_id)
    }

    pub fn remove(&mut self, img_id: &str) -> Option<Ann> {
        self.items.remove(img_id)
    }

    pub fn contains(&self, img_id: &str) -> bool {
        self.items.contains_key(img_id)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for AnnSet {
    type Item = Ann;
    type IntoIter = IntoValues<String, Ann>;

    /// Warning: O(capacity)
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_values()
    }
}

impl<'a> IntoIterator for &'a AnnSet {
    type Item = &'a Ann;
    type IntoIter = Values<'a, String, Ann>;

    /// Warning: O(capacity)
    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}

impl AnnSet {
    /// Warning: O(capacity)
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::annotationset::*;

    #[test]
    fn test_len() {
        let mut anns = AnnSet::new();
        let ann = Ann::new("image.jpg", None, vec![]);
        anns.insert(ann);

        assert!(anns.len() == 1);
    }
}