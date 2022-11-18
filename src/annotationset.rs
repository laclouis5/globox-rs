use std::collections::HashMap;
use crate::annotation::*;

pub type AnnSet = HashMap<String, Ann>;

#[cfg(test)]
mod tests {
    use crate::annotationset::*;

    #[test]
    fn test() {
        let a = AnnSet::new();
        assert!(a.is_empty());
    }
}