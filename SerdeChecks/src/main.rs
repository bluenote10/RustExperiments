use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::Compression;

use serde_checks::serialize_sequence;
use serde_checks::types::Sequence;

fn load_sequence_from_file(path: &Path) -> Sequence {
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn add_extension(path: &Path, extension: impl AsRef<std::path::Path>) -> PathBuf {
    // https://users.rust-lang.org/t/append-an-additional-extension/23586/12
    let mut path = path.to_owned();
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
    path
}

struct FileSize {
    raw: u64,
    zip: u64,
}

fn get_raw_file_size(path: &Path) -> u64 {
    let file = File::open(path).unwrap();
    file.metadata().unwrap().len()
}

fn get_zip_file_size(path: &Path) -> u64 {
    let file = File::open(path).unwrap();
    let mut input = BufReader::new(file);

    let path_out = add_extension(path, "gz");
    {
        // https://github.com/rust-lang/flate2-rs/blob/master/examples/compress_file.rs
        // Note that default compression corresponds to level 6:
        // https://docs.rs/flate2/1.0.22/src/flate2/lib.rs.html#223-225
        let compression = Compression::default();
        let output = File::create(&path_out).unwrap();
        let mut encoder = GzEncoder::new(output, compression);
        copy(&mut input, &mut encoder).unwrap();
        encoder.finish().unwrap();
    }
    get_raw_file_size(&path_out)
}

fn get_file_size(path: &Path) -> FileSize {
    FileSize {
        raw: get_raw_file_size(path),
        zip: get_zip_file_size(path),
    }
}

fn write_as_json(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.json");
    {
        serde_json::to_writer(File::create(path).unwrap(), seq).unwrap();
    }
    get_file_size(path)
}

fn write_as_pretty_json(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.pretty.json");
    {
        serde_json::to_writer_pretty(File::create(path).unwrap(), seq).unwrap();
    }
    get_file_size(path)
}

fn write_as_msgpack_compact(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.compact.msgpack");
    let data = rmp_serde::encode::to_vec(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_msgpack_named(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.named.msgpack");
    let data = rmp_serde::encode::to_vec_named(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_bincode(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.bincode");
    let data = bincode::serialize(seq).unwrap();
    {
        File::create(path).unwrap().write_all(&data).unwrap();
    }
    get_file_size(path)
}

fn write_as_cbor(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.cbor");
    {
        ciborium::ser::into_writer(seq, File::create(path).unwrap()).unwrap();
    }
    get_file_size(path)
}

fn write_as_bare(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.bare");
    {
        serde_bare::to_writer(File::create(path).unwrap(), seq).unwrap();
    }
    get_file_size(path)
}

fn write_as_custom(seq: &Sequence) -> FileSize {
    let path = Path::new("/tmp/test.custom");
    {
        serialize_sequence(seq, File::create(path).unwrap()).unwrap();
    }
    get_file_size(path)
}

fn main() {
    let path = Path::new("test.json");
    let seq = load_sequence_from_file(path);

    let size_orig = get_file_size(path);
    let size_json = write_as_json(&seq);
    let size_pretty_json = write_as_pretty_json(&seq);
    let size_msgpack_compact = write_as_msgpack_compact(&seq);
    let size_msgpack_named = write_as_msgpack_named(&seq);
    let size_bincode = write_as_bincode(&seq);
    let size_cbor = write_as_cbor(&seq);
    let size_bare = write_as_bare(&seq);
    let size_custom = write_as_custom(&seq);

    let entries = [
        ("orig", size_orig),
        ("json", size_json),
        ("pretty.json", size_pretty_json),
        ("msgpack (compact)", size_msgpack_compact),
        ("msgpack (named)", size_msgpack_named),
        ("bincode", size_bincode),
        ("cbor", size_cbor),
        ("bare", size_bare),
        ("custom", size_custom),
    ];

    let size_reference = 21749;
    let size_reference_zip = 4038;
    let get_ratio_to_ref = |len: u64| len as f64 / size_reference as f64;
    let get_ratio_to_ref_zip = |len: u64| len as f64 / size_reference_zip as f64;

    for (name, len) in entries {
        println!(
            "{:<20} {:10} {:8.3} {:10} {:8.3} {:8.3}",
            name.to_owned() + ":",
            len.raw,
            get_ratio_to_ref(len.raw),
            len.zip,
            get_ratio_to_ref(len.zip),
            get_ratio_to_ref_zip(len.zip),
        );
    }
}
