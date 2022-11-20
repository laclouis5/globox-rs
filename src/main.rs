use globox::{
    imgsize::ImgSize,
    annotation::Ann,
    annotationset::AnnSet, 
};

fn main() {
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/gts/yolo_format/obj_train_data/2007_000032.txt";
    
    let img_size = ImgSize::new(500, 281);  

    let ann = Ann::parse_yolo(path, img_size, false, "jpg").unwrap();
    println!("{:?}", ann);
}