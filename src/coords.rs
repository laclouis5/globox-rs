use crate::imgsize::*;

pub type Coords = (f32, f32, f32, f32);

pub fn ltwh_to_ltrb(coords: Coords) -> Coords {
    let (l, t, w, h) = (coords.0, coords.1, coords.2, coords.3);
    let r = l + w;
    let b = t + h;
    (l, t, r, b)
}

pub fn xywh_to_ltrb(coords: Coords) -> Coords {
    let (x, y, w, h) = (coords.0, coords.1, coords.2, coords.3);
    
    let hw = w / 2.0;
    let hh = h / 2.0;
    
    let l = x - hw;
    let t = y - hh;
    let r = x + hw;
    let b = y + hh;
    (l, t, r, b)
}

pub fn abs_to_rel(coords: Coords, size: ImgSize) -> Coords {
    let (a, b, c, d) = (coords.0, coords.1, coords.2, coords.3);
    
    let w = size.width as f32;
    let h = size.height as f32;
    
    (a / w, b / h, c / w, d / h)
}

pub fn rel_to_abs(coords: Coords, size: ImgSize) -> Coords {
    let (a, b, c, d) = (coords.0, coords.1, coords.2, coords.3);
    
    let w = size.width as f32;
    let h = size.height as f32;
    
    (a * w, b * h, c * w, d * h)
}