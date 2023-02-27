use anime_filename_parser::parse_filename;

fn main() {
    let filename = "[test] test";
    let metadata = parse_filename(filename).unwrap();
    println!("{:?}", metadata);
}
