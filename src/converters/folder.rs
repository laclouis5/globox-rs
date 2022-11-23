use crate::{
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError, 
};

impl AnnSet {
    /// Save all the annotations given a save function operating
    /// on individual annotations.
    pub fn save_all<F>(
        &self,
        save_fn: F,
    ) -> Result<(), ConvError> where 
        F: Fn(&Ann) -> Result<(), ConvError>,
    {

        for ann in self {
            save_fn(ann)?;
        }

        Ok(())
    }
}