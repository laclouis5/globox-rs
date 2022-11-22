use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct OALine<'l> {
    #[serde(rename = "ImageID")]
    pub(crate) img_id: &'l str,

    #[serde(rename = "Source")]
    pub(crate) source: Option<&'l str>,

    #[serde(rename = "LabelName")]
    pub(crate) label: &'l str,

    #[serde(rename = "Confidence")]
    pub(crate) conf: Option<f32>,

    #[serde(rename = "XMin")]
    pub(crate) xmin: f32,

    #[serde(rename = "YMin")]
    pub(crate) ymin: f32,

    #[serde(rename = "XMax")]
    pub(crate) xmax: f32,

    #[serde(rename = "YMax")]
    pub(crate) ymax: f32,

    #[serde(rename = "IsOccluded")]
    pub(crate) is_occulded: Option<bool>,

    #[serde(rename = "IsTruncated")]
    pub(crate) is_trucated: Option<bool>,

    #[serde(rename = "IsGroupOf")]
    pub(crate) is_group_of: Option<bool>,

    #[serde(rename = "IsDepiction")]
    pub(crate) is_depiction: Option<bool>,

    #[serde(rename = "IsInside")]
    pub(crate) is_inside: Option<bool>,
}

impl<'l> OALine<'l> {
    pub(crate) fn new(
        img_id: &'l str, 
        label: &'l str, 
        xmin: f32, 
        ymin: f32, 
        xmax: f32, 
        ymax: f32, 
        conf: Option<f32>
    ) -> OALine<'l> {
        OALine { 
            img_id,
            source: None,
            label,
            conf, 
            xmin, ymin, xmax, ymax,
            is_occulded: None,
            is_trucated: None,
            is_group_of: None,
            is_depiction: None,
            is_inside: None
        }
    }
}