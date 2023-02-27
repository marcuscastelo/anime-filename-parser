use anime_filename_parser::parse_filename;

fn main() {
    let filename = "[test] test".to_owned();
    let metadata = parse_filename(filename).unwrap();
    println!("{:?}", metadata);
}
