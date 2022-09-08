use anyhow::bail;
use tch::{vision::imagenet, Tensor};

///
/// cargo run -- ~/gdrive/colab/ddsp_pytorch/output/debug/state.pth /tmp/out
///
/// Internal torch error: PytorchStreamReader failed locating file constants.pkl: file not found
/// https://stackoverflow.com/questions/69979034/runtimeerror-pytorchstreamreader-failed-locating-file-data-pkl-file-not-found
///


fn hello_world() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
}

/*
https://github.com/LaurentMazare/tch-rs/tree/main/examples/jit
*/

pub fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let (model_file, image_file) = match args.as_slice() {
        [_, m, i] => (m.to_owned(), i.to_owned()),
        _ => bail!("usage: main model.pt image.jpg"),
    };
    let model = tch::CModule::load(model_file)?;
    /*
    let output = model
        .forward_ts(&[image.unsqueeze(0)])?
        .softmax(-1, tch::Kind::Float);
    for (probability, class) in imagenet::top(&output, 5).iter() {
        println!("{:50} {:5.2}%", class, 100.0 * probability)
    }
    */
    Ok(())
}
