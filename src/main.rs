use globox::annotation::Ann;

fn main() {
    let path = "/Users/louislac/Documents/Developer/Python/globox/tests/globox_test_data/gts/labelme_format/2007_000032.json";

    let ann = Ann::from_labelme(path).unwrap();
    println!("{:?}", ann);
}