use globox::{annotation::Ann, annotationset::AnnSet};

fn main() {
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/gts/cvat_format/annotations.xml";

    let ann = AnnSet::parse_cvat(path).unwrap();
    println!("{:?}", ann);
}