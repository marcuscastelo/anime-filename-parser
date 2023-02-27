use anime_filename_parser::parse_filename;

fn main() {
    let filename = "[test] Anime adsf".to_owned();
    let metadata = parse_filename(filename).unwrap();
    println!("{:?}", metadata);
}
