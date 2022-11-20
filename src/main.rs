use globox::{annotationset::AnnSet};

fn main() {
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/gts/openimages_format/all_bounding_boxes.csv";
    let imgs_path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/images";

    let ann = AnnSet::parse_openimage(path, imgs_path).unwrap();
    println!("{:?}", ann);
}