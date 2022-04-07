mod types;

use std::fs::File;
use std::io::Write;
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
    {
        serde_json::to_writer(File::create(path).unwrap(), seq).unwrap();
    }
    get_file_size(path)
}

fn write_as_pretty_json(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.pretty.json");
    {
        serde_json::to_writer_pretty(File::create(path).unwrap(), seq).unwrap();
    }
    get_file_size(path)
}

fn write_as_msgpack_compact(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.compact.msgpack");
    let data = rmp_serde::encode::to_vec(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_msgpack_named(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.named.msgpack");
    let data = rmp_serde::encode::to_vec_named(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_bincode(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.bincode");
    let data = bincode::serialize(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_cbor(seq: &Sequence) -> u64 {
    let path = Path::new("/tmp/test.cbor");
    {
        ciborium::ser::into_writer(seq, File::create(path).unwrap()).unwrap();
    }
    get_file_size(path)
}

// rmp_serde::encode::to_vec

fn main() {
    let path = Path::new("test.json");
    let seq = load_sequence_from_file(path);

    let len_reference = 21749;

    let len_orig = get_file_size(path);
    let len_json = write_as_json(&seq);
    let len_pretty_json = write_as_pretty_json(&seq);
    let len_msgpack_compact = write_as_msgpack_compact(&seq);
    let len_msgpack_named = write_as_msgpack_named(&seq);
    let len_bincode = write_as_bincode(&seq);
    let len_cbor = write_as_cbor(&seq);

    let get_ref_size = |len: u64| len as f64 / len_reference as f64;

    let entries = [
        ("orig", len_orig),
        ("json", len_json),
        ("pretty.json", len_pretty_json),
        ("msgpack (compact)", len_msgpack_compact),
        ("msgpack (named)", len_msgpack_named),
        ("bincode", len_bincode),
        ("cbor", len_cbor),
    ];

    for (name, len) in entries {
        println!(
            "{:<20} {:10} {:8.3}",
            name.to_owned() + ":",
            len,
            get_ref_size(len)
        );
    }
}
