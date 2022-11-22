use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct INetBndBox {
    pub(crate) xmin: f32, 
    pub(crate) ymin: f32, 
    pub(crate) xmax: f32, 
    pub(crate) ymax: f32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct InetSize {
    pub(crate) width: u32, 
    pub(crate) height: u32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct InetObj {
    pub(crate) name: String, 
    pub(crate) bndbox: INetBndBox,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct InetAnn {
    pub(crate) filename: String, 
    pub(crate) size: InetSize, 

    #[serde(rename = "object")]
    pub(crate) objects: Vec<InetObj>,
}
