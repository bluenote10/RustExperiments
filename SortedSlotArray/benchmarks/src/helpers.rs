use std::fs::{create_dir_all, File};
use std::path::Path;
use serde_json::json;

use std::process::{Command, Stdio};

use rand::seq::SliceRandom;
use rand::thread_rng;


pub fn export_elapsed_times(name: &str, run: i32, filename: &str, iters: &[usize], times: &[f64]) {

    let json_data = json!({
        "name": name,
        "run": run,
        "iters": iters,
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

    let mut child = Command::new(script_path.as_os_str())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to run Python plot.");

    child.wait().expect("Failed to wait for child");
}

pub fn export_stats(iters: &[usize], times: &[f64], fill_ratio: &[f64], num_blocks: &[usize], capacity: &[u16]) {

    let json_data = json!({
        "iters": iters,
        "times": times,
        "fill_ratio": fill_ratio,
        "num_blocks": num_blocks,
        "capacity": capacity,
    });

    let path = Path::new("results/fill_stats.json");
    let parent = path.parent().unwrap();
    create_dir_all(parent).unwrap();

    let f = File::create(path).expect("Unable to create json file.");
    serde_json::to_writer_pretty(f, &json_data).expect("Unable to write json file.");
}

pub fn shuffle<T>(v: &[T]) -> Vec<T>
where
    T: Clone
{
    let mut rng = thread_rng();
    let mut v_cloned = v.to_vec();
    v_cloned.shuffle(&mut rng);
    v_cloned
}