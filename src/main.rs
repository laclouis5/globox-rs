use globox::{
    imgsize::ImgSize,
    annotation::Ann,
    annotationset::AnnSet, 
};

fn main() {
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/gts/coco_format_v1/instances_default.json";

    let ann = AnnSet::parse_coco(path).unwrap();
    println!("{:?}", ann);
}