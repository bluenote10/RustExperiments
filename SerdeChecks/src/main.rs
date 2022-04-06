mod types;

use std::fs::File;
use std::path::Path;

use types::Sequence;

fn load_sequence_from_file(path: &Path) -> Sequence {
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn get_file_size(path: &Path) -> u64 {
    let file = File::open(path).unwrap();
    file.metadata().unwrap().len()
}

fn write_as_json(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.json");
    serde_json::to_writer(File::create(path).unwrap(), seq).unwrap();
    get_file_size(path)
}

fn write_as_pretty_json(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.pretty.json");
    serde_json::to_writer_pretty(File::create(path).unwrap(), seq).unwrap();
    get_file_size(path)
}

fn main() {
    let path = Path::new("test.json");
    let seq = load_sequence_from_file(path);

    let len_reference = 21749;
    let len_orig = get_file_size(path);
    let len_json = write_as_json(&seq);
    let len_pretty_json = write_as_pretty_json(&seq);

    let get_ref_size = |len: u64| len as f64 / len_reference as f64;

    println!(
        "orig:        {:10} {:8.3}",
        len_orig,
        get_ref_size(len_orig)
    );
    println!(
        "json:        {:10} {:8.3}",
        len_json,
        get_ref_size(len_json)
    );
    println!(
        "pretty.json: {:10} {:8.3}",
        len_pretty_json,
        get_ref_size(len_pretty_json)
    );
}
