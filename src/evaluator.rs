use crate::annotationset::*;

pub struct COCOEval<'e> {
    gts: &'e AnnSet,
    dets: &'e AnnSet,
}

impl<'e> COCOEval<'e> {
    pub fn new(gts: &'e AnnSet, dets: &'e AnnSet) -> COCOEval<'e> {
        COCOEval { gts, dets }
    }
}

impl COCOEval<'_> {
    fn validate_args(thresh: f32, ndets: u32, sz_range: (f32, f32)) -> bool {
        let (low, high) = sz_range;
        
        0.0 <= thresh && thresh <= 1.0 && 0.0 <= low && low <= high
    }

    pub fn eval(&self, thresh: f32, ndets: u32, sz_range: (f32, f32)) {
        todo!()
    }

    pub fn eval_all(&self) {
        todo!()
    }

    pub fn summarize(&self) {
        todo!()
    }
}