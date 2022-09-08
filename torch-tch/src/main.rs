use std::time::Instant;

use anyhow::bail;
use tch::{Device, Kind, Tensor};

///
/// https://docs.rs/tch/latest/tch/index.html
/// https://github.com/LaurentMazare/tch-rs/tree/main/examples/jit
///
/// cargo run -- ~/git/_ext/ddsp_pytorch/export/ddsp_debug_pretrained.ts
///

pub fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let model_file = match args.as_slice() {
        [_, m] => (m.to_owned()),
        _ => bail!("usage: main <TORCHSCRIPT-MODEL>"),
    };
    let model = tch::CModule::load(model_file)?;

    println!("is_cuda: {}", Device::cuda_if_available().is_cuda());

    let device = Device::Cpu;

    let num_frames = 10;
    let num_steps = 100;

    let t_start = Instant::now();
    let mut num_samples = 0;
    for _ in 0..num_frames {
        let pitches = Tensor::rand(&[num_steps], (Kind::Float, device));
        let pitches = pitches.reshape(&[1, -1, 1]);
        let loudness = Tensor::rand(&[num_steps], (Kind::Float, device));
        let loudness = loudness.reshape(&[1, -1, 1]);
        println!("loudness.shape = {:?}", loudness.size());
        println!("pitches.shape = {:?}", pitches.size());

        let output = model.forward_ts(&[&pitches, &loudness])?;
        // output.print();

        println!("output.shape = {:?}", output.size());
        num_samples += output.size3()?.1;
    }

    let elapsed = t_start.elapsed();
    println!("Total samples: {}", num_samples);
    println!("Total time: {}", elapsed.as_secs_f64());
    println!(
        "Throughput: {} kb / sec",
        (num_samples as f64) / 1000.0 / elapsed.as_secs_f64()
    );

    Ok(())
}
