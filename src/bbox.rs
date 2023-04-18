use crate::imgsize::ImgSize;
use crate::coords::{
    Coords,
    rel_to_abs,
    ltwh_to_ltrb,
    xywh_to_ltrb,
};

/// A rectangular bounding box with a label and an optional 
/// confidence score.
/// 
/// The bounding box coordinates are expressed in pixels where 
/// `(xmin, ymin)` is the top-left corner and `(xmax, ymax)` the 
/// bottom-right corner.
#[derive(Debug, Clone)]
pub struct BBox {
    /// The bounding box label.
    pub label: String,
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
    pub fn new<L: Into<String>>(
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
    pub fn create<L: Into<String>>(
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
    pub fn create_rel<L: Into<String>>(
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
    /// The bounding box top-left corner X coordinate.
    pub fn xmin(&self) -> f32 { self.xmin }

    /// The bounding box top-left corner Y coordinate.
    pub fn ymin(&self) -> f32 { self.ymin }

    /// The bounding box bottom-right corner X coordinate.
    pub fn xmax(&self) -> f32 { self.xmax }

    /// The bounding box bottom-right corner Y coordinate.
    pub fn ymax(&self) -> f32 { self.ymax }

    /// The bounding box center X coordinate.
    pub fn xmid(&self) -> f32 { (self.xmax + self.xmin) / 2.0 }

    /// The bounding box center X coordinate.
    pub fn ymid(&self) -> f32 { (self.ymax + self.ymin) / 2.0 }

    /// The bounding box width.
    pub fn width(&self) -> f32 { self.xmax - self.xmin }

    /// The bounding box height.
    pub fn height(&self) -> f32 { self.ymax - self.ymin }

    /// The bounding box confidence score.
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

impl BBox {
    pub fn is_ground_truth(&self) -> bool {
        self.conf.is_none()
    }

    pub fn is_detection(&self) -> bool {
        self.conf.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::bbox::*;

    #[test]
    #[should_panic]
    fn xmax_lt_xmin() {
        let _bbox = BBox::new("", 10.0, 10.0, 5.0, 10.0, None);
    }

    #[test]
    #[should_panic]
    fn ymax_lt_ymin() {
        let _bbox = BBox::new("", 10.0, 10.0, 15.0, 9.0, None);
    }

    #[test]
    #[should_panic]
    fn neg_conf() {
        let _bbox = BBox::new("", 0.0, 0.0, 0.0, 0.0, Some(-0.1));
    }

    #[test]
    #[should_panic]
    fn conf_gt_one() {
        let _bbox = BBox::new("", 0.0, 0.0, 0.0, 0.0, Some(1.1));
    }

    #[test]
    fn accessors() {
        let bbox = BBox::new("", -1.0, 0.0, 1.0, 2.0, None);

        assert!(bbox.xmin() == -1.0);
        assert!(bbox.ymin() == 0.0);
        assert!(bbox.xmax() == 1.0);
        assert!(bbox.ymax() == 2.0);

        assert!(bbox.xmid() == 0.0);
        assert!(bbox.ymid() == 1.0);

        assert!(bbox.width() == 2.0);
        assert!(bbox.height() == 2.0);

        assert!(bbox.conf.is_none());

        assert!(bbox.area() == 4.0); 

        assert!(bbox.is_ground_truth());
        assert!(!bbox.is_detection());

        assert!(bbox.ltrb() == (-1.0, 0.0, 1.0, 2.0));
        assert!(bbox.ltwh() == (-1.0, 0.0, 2.0, 2.0));
        assert!(bbox.xywh() == (0.0, 1.0, 2.0, 2.0));

        assert!(bbox.coords(BBoxFmt::LTRB) == bbox.ltrb());
        assert!(bbox.coords(BBoxFmt::LTWH) == bbox.ltwh());
        assert!(bbox.coords(BBoxFmt::XYWH) == bbox.xywh());
    }

    #[test]
    fn set_conf() {
        let mut bbox = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);
        assert!(bbox.conf().is_none());

        bbox.set_conf(0.25);
        assert!(bbox.conf() == Some(0.25));
    }

    #[test]
    #[should_panic]
    fn set_invalid_neg_conf() {
        let mut bbox = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);

        bbox.set_conf(-0.1);
    }

    #[test]
    #[should_panic]
    fn set_invalid_gt_one_conf() {
        let mut bbox = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);

        bbox.set_conf(1.1);
    }

    #[test]
    fn create_rel_ltrb() {
        let bbox = BBox::create_rel("", (0.0, 0.0, 0.75, 1.0), BBoxFmt::LTRB, None, ImgSize::new(100, 200));

        assert!(bbox.xmin() == 0.0);
        assert!(bbox.ymin() == 0.0);
        assert!(bbox.xmax() == 75.0);
        assert!(bbox.ymax() == 200.0);
        assert!(bbox.xmid() == 37.5);
        assert!(bbox.ymid() == 100.0);
        assert!(bbox.width() == 75.0);
        assert!(bbox.height() == 200.0);
    }

    #[test]
    fn create_rel_ltwh() {
        let bbox = BBox::create_rel("", (0.0, 0.0, 0.5, 1.0), BBoxFmt::LTWH, None, ImgSize::new(100, 200));

        assert!(bbox.xmin() == 0.0);
        assert!(bbox.ymin() == 0.0);
        assert!(bbox.xmax() == 50.0);
        assert!(bbox.ymax() == 200.0);
        assert!(bbox.xmid() == 25.0);
        assert!(bbox.ymid() == 100.0);
        assert!(bbox.width() == 50.0);
        assert!(bbox.height() == 200.0);
    }

    #[test]
    fn create_rel_xywh() {
        let bbox = BBox::create_rel("", (0.5, 0.5, 0.5, 1.0), BBoxFmt::XYWH, None, ImgSize::new(100, 200));

        assert!(bbox.xmin() == 25.0);
        assert!(bbox.ymin() == 0.0);
        assert!(bbox.xmax() == 75.0);
        assert!(bbox.ymax() == 200.0);
        assert!(bbox.xmid() == 50.0);
        assert!(bbox.ymid() == 100.0);
        assert!(bbox.width() == 50.0);
        assert!(bbox.height() == 200.0);
    }

    // #[test]
    // fn iou() {
    //     let bbox1 = BBox::new("", 0.0, 0.0, 10.0, 10.0, None);
    //     let bbox2 = BBox::new("", 5.0, 0.0, 10.0, 10.0, None);

    //     // assert!(bbox1.iou)
    // }
}