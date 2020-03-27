use std::fs::{create_dir_all, File};
use std::path::Path;
use serde_json::json;

use std::process::{Command, Stdio};

pub fn export_elapsed_times(name: &str, filename: &str, times: &[f64]) {

    let json_data = json!({
        "name": name,
        "times": times,
    });

    let path = Path::new(filename);
    let parent = path.parent().unwrap();
    create_dir_all(parent).unwrap();

    let f = File::create(path).expect("Unable to create json file.");
    serde_json::to_writer_pretty(f, &json_data).expect("Unable to write json file.");
}

pub fn call_plots() {
    let script_path = Path::new(file!()).to_path_buf()
        .canonicalize().unwrap()
        .parent().unwrap().to_path_buf() // -> /src
        .parent().unwrap().to_path_buf() // -> /
        .join("scripts")
        .join("plot.py");

    Command::new(script_path.as_os_str())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to run Python plot.");
}