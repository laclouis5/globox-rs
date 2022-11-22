use globox::cli::run;

fn main() {
    // run();

    use globox::annotationset::AnnSet;
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/coco_val_5k/coco.json";
    let save_dir = "/Users/louislac/Downloads/openimage.csv";

    let anns = AnnSet::parse_coco(path).unwrap();
    anns.save_openimage(save_dir).unwrap();
}