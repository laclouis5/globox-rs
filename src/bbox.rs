use crate::imgsize::*;
use crate::coords::*;

#[derive(Clone)]
pub struct BBox {
    pub label: String,  // Use SmallString or immutable to optimize storage?
    xmin: f32,
    ymin: f32,
    xmax: f32,
    ymax: f32,
    conf: Option<f32>,
}

impl BBox {
    pub fn new(label: &str, xmin: f32, ymin: f32, xmax: f32, ymax: f32, conf: Option<f32>) -> Self {
        assert!(xmin <= xmax);
        assert!(ymin <= ymax);
        
        if let Some(conf) = conf {
            assert!(0.0 <= conf && conf <= 1.0);
        }

        Self { label: String::from(label), xmin, ymin, xmax, ymax, conf }
    }
}

impl BBox {
    pub fn xmin(&self) -> f32 { self.xmin }
    pub fn ymin(&self) -> f32 { self.ymin }
    pub fn xmax(&self) -> f32 { self.xmax }
    pub fn ymax(&self) -> f32 { self.ymax }

    pub fn xmid(&self) -> f32 { (self.xmax + self.xmin) / 2.0 }
    pub fn ymid(&self) -> f32 { (self.ymax + self.ymin) / 2.0 }

    pub fn width(&self) -> f32 { self.xmax - self.xmin }
    pub fn height(&self) -> f32 { self.ymax - self.ymin }

    pub fn conf(&self) -> Option<f32> { self.conf }
}

impl BBox {
    pub fn ltrb(&self) -> Coords {
        (self.xmin(), self.ymin(), self.xmax(), self.ymax())
    }

    pub fn ltwh(&self) -> Coords {
        (self.xmin(), self.ymin(), self.width(), self.height())
    }

    pub fn xywh(&self) -> Coords {
        (self.xmid(), self.ymid(), self.width(), self.height())
    }
}

#[derive(Clone, Copy)]
pub enum BBoxFmt {
    LTRB, 
    LTWH, 
    XYWH,
}

impl BBox {
    pub fn create(
        label: &str, 
        coords: Coords, 
        fmt: BBoxFmt, 
        conf: Option<f32>, 
    ) -> Self {
        let (xmin, ymin, xmax, ymax) = match fmt {
            BBoxFmt::LTRB => coords,
            BBoxFmt::LTWH => ltwh_to_ltrb(coords),
            BBoxFmt::XYWH => xywh_to_ltrb(coords),
        };

        BBox::new(label, xmin, ymin, xmax, ymax, conf)
    }

    pub fn create_rel(
        label: &str, 
        coords: Coords, 
        fmt: BBoxFmt, 
        conf: Option<f32>,
        imgsize: ImgSize,
    ) -> Self {
        let coords = rel_to_abs(coords, imgsize);
        BBox::create(label, coords, fmt, conf)
    }
}

#[cfg(test)]
mod tests {
    use crate::bbox::*;

    #[test]
    fn test_creation() {
        let b = BBox::new("label", 10.0, 20.0, 30.0, 40.0, Some(0.25));
        
        assert_eq!(b.xmin(), 10.0);
        assert_eq!(b.ymin(), 20.0);
        assert_eq!(b.xmax(), 30.0);
        assert_eq!(b.ymax(), 40.0);

        assert_eq!(b.xmid(), 20.0);
        assert_eq!(b.ymid(), 30.0);

        assert_eq!(b.width(), 20.0);
        assert_eq!(b.height(), 20.0);

        assert_eq!(b.conf(), Some(0.25));
    }
}