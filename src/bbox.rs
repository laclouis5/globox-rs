use crate::imgsize::ImgSize;
use crate::coords::{
    Coords,
    rel_to_abs,
    ltwh_to_ltrb,
    xywh_to_ltrb,
};

use smol_str::SmolStr;

/// A rectangular bounding box with a label and an optional 
/// confidence score.
/// 
/// The bounding box coordinates are expressed in pixels where 
/// `(xmin, ymin)` is the top-left corner and `(xmax, ymax)` the 
/// bottom-right corner.
#[derive(Debug, Clone)]
pub struct BBox {
    /// The bounding box label.
    pub label: SmolStr,
    xmin: f32, ymin: f32,
    xmax: f32, ymax: f32,
    conf: Option<f32>,
}

impl BBox {
    /// Creates a bounding box annotation.
    /// 
    /// The coordinates are expressed in pixels where `(xmin, ymin)` is
    /// the top-left corner and `(xmax, ymax)` the bottom-right corner.
    /// 
    /// # Panics
    /// Will panic if the confidence score is not in `0..=1` or 
    /// if the coordinates are invalid.
    pub fn new<L: Into<SmolStr>>(
        label: L, 
        xmin: f32, 
        ymin: f32, 
        xmax: f32, 
        ymax: f32, 
        conf: Option<f32>
    ) -> Self {
        assert!(
            xmin <= xmax, 
            "`xmax` ({}) should be greater than or equal to `xmin` ({})", xmax, xmin
        );
        assert!(
            ymin <= ymax, 
            "`ymax` ({}) should be greater than or equal to `ymin` ({})", ymax, ymin
        );
        
        if let Some(conf) = conf {
            assert!(
                0.0 <= conf && conf <= 1.0, 
                "confidence score ({}) should be in 0..=1", conf
            );
        }

        Self { label: label.into(), xmin, ymin, xmax, ymax, conf }
    }

    /// Creates a bounding box annotation in the given coordinate format.
    /// 
    /// # Panics
    /// Will panic if the confidence score is not in `0..=1` or 
    /// if the coordinates are invalid.
    pub fn create<L: Into<SmolStr>>(
        label: L, 
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

    /// Creates a bounding box in the given coordinate format using
    /// relative coordinates.
    /// 
    /// The image size must be provided to convert the coordinates 
    /// to be absolute.
    /// 
    /// # Panics
    /// Will panic if the confidence score is not in `0..=1` or 
    /// if the coordinates are invalid.
    pub fn create_rel<L: Into<SmolStr>>(
        label: L, 
        coords: Coords,
        fmt: BBoxFmt, 
        conf: Option<f32>,
        img_size: ImgSize,
    ) -> Self {
        let coords = rel_to_abs(coords, img_size);
        BBox::create(label, coords, fmt, conf)
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

    /// Sets the bounding box confidence score.
    /// 
    /// # Panics
    /// The operation panics if the score is not in `0..=1`.
    pub fn set_conf(&mut self, conf: f32) {
        assert!(0.0 <= conf && conf <= 1.0);
        self.conf = Some(conf);
    }
}

/// The coordinates format of a bounding box.
/// 
/// The available formats are:
/// * `BBoxFormat::LTRB`: 
///`(xmin, ymin, xmax, ymax)` where `(xmin, ymin)` is
/// the top-left corner and `(xmax, ymax)` the bottom-right one.
/// 
/// * `BBoxFormat::LTWH`:
/// `(xmin, ymin, width, height)` where `(xmin, ymin)` is
/// the top-left corner and `(width, height)` the bounding box size.
/// 
/// * `BBoxFormat::XYWH`:
/// `(xmid, ymid, width, height)` where `(xmid, ymid)` is
/// bounding box center and `(width, height)` the bounding box size.
#[derive(Clone, Copy)]
pub enum BBoxFmt {
    /// `(xmin, ymin, xmax, ymax)` where `(xmin, ymin)` is
    /// the top-left corner and `(xmax, ymax)` the bottom-right one.
    LTRB, 
    /// `(xmin, ymin, width, height)` where `(xmin, ymin)` is
    /// the top-left corner and `(width, height)` the bounding box size.
    LTWH, 
    /// `(xmid, ymid, width, height)` where `(xmid, ymid)` is
    /// bounding box center and `(width, height)` the bounding box size.
    XYWH,
}

impl BBox {
    /// Returns the bounding box coordinates as a `(xmin, ymin, xmax, ymax)` 
    /// tuple.
    pub fn ltrb(&self) -> Coords {
        (self.xmin(), self.ymin(), self.xmax(), self.ymax())
    }

    /// Returns the bounding box coordinates as a `(xmin, ymin, width, height)` 
    /// tuple.
    pub fn ltwh(&self) -> Coords {
        (self.xmin(), self.ymin(), self.width(), self.height())
    }

    /// Returns the bounding box coordinates as a `(xmid, ymid, width, height)` 
    /// tuple.
    pub fn xywh(&self) -> Coords {
        (self.xmid(), self.ymid(), self.width(), self.height())
    }

    /// Returns the bounding box coordinates in the specified coordinate format.
    pub fn coords(&self, fmt: BBoxFmt) -> Coords {
        match fmt {
            BBoxFmt::LTRB => self.ltrb(),
            BBoxFmt::LTWH => self.ltwh(),
            BBoxFmt::XYWH => self.xywh(),
        }
    }
}

impl BBox {
    /// The bounding box area.
    pub fn area(&self) -> f32 {
        self.width() * self.height()
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