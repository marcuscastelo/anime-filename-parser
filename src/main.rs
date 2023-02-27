use anime_filename_parser::parse_filename;

fn main() {
    let filename = "[Ohys-Raws] Bocchi the Rock! - 08 (BS11 1280x720 x264 AAC).mp4".to_owned();
    let parse_result = parse_filename(filename);
    match parse_result {
        Ok(metadata) => println!("{:#?}", metadata),
        Err(e) => println!("\n{e:?}"),
    }
}
