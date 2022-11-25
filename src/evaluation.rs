use crate::bbox::BBox;

/// Compute the Intersection over Union (IoU) of 
/// two bounding boxes.
pub fn iou(lhs: &BBox, rhs: &BBox) -> f32 {
    let xmin = lhs.xmin().max(rhs.xmin());
    let ymin = lhs.ymin().max(rhs.ymin());
    let xmax = lhs.xmax().min(rhs.xmax());
    let ymax = lhs.ymax().min(rhs.ymax());

    // The bounding boxes do not intersect.
    if xmax <= xmin || ymax <= ymin {
        return 0.0
    }

    let inter = (xmax - xmin) * (ymax - ymin);

    let uni = lhs.area() + rhs.area() - inter;
    
    // `uni` is never equal to zero since it implies that
    // one of the bounding box has an area of zero and this
    // implies in turns that xmin == xmax and ymin == ymax, thus
    // the function would have returned early.
    return inter / uni
}

#[cfg(test)]
mod tests {
    use crate::bbox::*;
    use crate::evaluation::*;

    #[test]
    fn test_iou_intersect_non_null() {
        let b1 = BBox::new("", 0.0, 0.0, 30.0, 40.0, None);
        let b2 = BBox::new("", 10.0, 20.0, 50.0, 60.0, None);

        assert_ne!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_intersect_null() {
        let b1 = BBox::new("", 0.0, 0.0, 30.0, 40.0, None);
        let b2 = BBox::new("", 40.0, 50.0, 60.0, 70.0, None);

        assert_eq!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_area_null() {
        let b1 = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);
        let b2 = BBox::new("", 10.0, 20.0, 50.0, 60.0, None);

        assert_eq!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_area_null_in() {
        let b1 = BBox::new("", 20.0, 30.0, 20.0, 30.0, None);
        let b2 = BBox::new("", 0.0, 0.0, 40.0, 60.0, None);

        assert_eq!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_concat() {
        let b1 = BBox::new("", 0.0, 0.0,10.0, 10.0, None);
        let b2 = BBox::new("", 10.0, 0.0, 20.0, 10.0, None);

        assert_eq!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_box_in() {
        let b1 = BBox::new("", 10.0, 10.0, 20.0, 20.0, None);
        let b2 = BBox::new("", 0.0, 0.0, 30.0, 30.0, None);

        assert_ne!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_bboxes_equal_not_null() {
        let b1 = BBox::new("", 10.0, 10.0, 20.0, 20.0, None);
        let b2 = BBox::new("", 10.0, 10.0, 20.0, 20.0, None);

        assert_eq!(iou(&b1, &b2), 1.0);
    }

    #[test]
    fn test_iou_bboxes_equal_null() {
        let b1 = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);
        let b2 = BBox::new("", 0.0, 0.0, 0.0, 0.0, None);

        assert_eq!(iou(&b1, &b2), 0.0);
    }

    #[test]
    fn test_iou_value() {
        let b1 = BBox::new("", 0.0, 0.0, 10.0, 30.0, None);
        let b2 = BBox::new("", 0.0, 10.0, 10.0, 40.0, None);

        assert_eq!(iou(&b1, &b2), 0.5);
    }    
}