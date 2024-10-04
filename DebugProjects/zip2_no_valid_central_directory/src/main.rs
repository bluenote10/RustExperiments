fn main() {
    let file_path = std::path::Path::new("/tmp/test_small.zip");
    let file = std::fs::File::open(file_path).unwrap();
    let zip = zip::ZipArchive::new(file).unwrap();
    println!("Number of files: {}", zip.len());
}
