use std::{
    collections::{HashMap, hash_map::{IntoValues, Values}},
};
use crate::annotation::Ann;

use smol_str::SmolStr;

/// A set of annotations with efficient indexing by image id.
/// 
/// Annotations cannot be modified in-place. Modification is
/// achived through a remove-insert operation or by consuming
/// and mapping.
#[derive(Debug, Clone)]
pub struct AnnSet {
    /// WARNING: do not mutate the annotation labels.
    pub(crate) items: HashMap<SmolStr, Ann>,
}

impl AnnSet {
    /// Creates an empty set of annotations.
    pub fn new() -> AnnSet {
        AnnSet { items: HashMap::new() }
    }

    /// Creates an empty set of annotations with at least the specified
    /// capacity.
    pub fn with_capacity(capacity: usize) -> AnnSet {
        AnnSet { items: HashMap::with_capacity(capacity) }
    }

    /// Reserves capacity for at least `additional` annotations.
    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional)
    }
}

impl AnnSet {
    // TODO: Avoid cloning for the key.
    /// Inserts an annotation into the annotation set.
    /// 
    /// If an annotation with the same image identifier was
    /// already present, the old value is returned.
    pub fn insert(&mut self, ann: Ann) -> Option<Ann> {
        self.items.insert(ann.img_id.clone(), ann)
    }

    /// Returns a reference to the annotation corresponding to the
    /// specified image identifier.
    pub fn get(&self, img_id: &str) -> Option<&Ann> {
        self.items.get(img_id)
    }

    /// WARNING: do not mutate the annotation label as it
    /// breaks the invariant.
    pub(crate) fn get_mut(&mut self, img_id: &str) -> Option<&mut Ann> {
        self.items.get_mut(img_id)
    }

    /// Removes and returns the annotation having the specified image 
    /// identifier from the annotation set.
    pub fn remove(&mut self, img_id: &str) -> Option<Ann> {
        self.items.remove(img_id)
    }

    /// Returns true if the annotation set contains an annotation with 
    /// the specified image identifier.
    pub fn contains(&self, img_id: &str) -> bool {
        self.items.contains_key(img_id)
    }

    /// Returns the number of bounding box annotations in the annotation set.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for AnnSet {
    type Item = Ann;
    type IntoIter = IntoValues<SmolStr, Ann>;

    // TODO: O(capacity)
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_values()
    }
}

impl<'a> IntoIterator for &'a AnnSet {
    type Item = &'a Ann;
    type IntoIter = Values<'a, SmolStr, Ann>;

    // TODO: O(capacity)
    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}

impl AnnSet {
    // TODO: O(capacity)
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