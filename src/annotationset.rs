use std::collections::HashMap;
use crate::annotation::*;

type AnnSet = HashMap<String, Ann>;

// #[derive(Clone)]
// pub struct AnnSet {
//     anns: HashMap<String, Ann>,
// }

// impl AnnSet {
//     pub fn new(anns: HashMap<String, Ann>) -> Self {
//         AnnSet { anns }
//     }
    
//     pub fn empty() -> Self {
//         AnnSet { anns: HashMap::new() }
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         AnnSet { anns: HashMap::with_capacity(capacity) }
//     }
// }

// impl AnnSet {
//     pub fn contains_id(&self, img_id: &str) -> bool {
//         self.anns.contains_key(img_id)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::annotationset::*;

    #[test]
    fn test() {
        let a = AnnSet::new();
        assert!(a.is_empty());
    }
}