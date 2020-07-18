use std::ops::Index;
use std::ops::IndexMut;

fn compute<In, Out>(_num_to_generate: usize, in_buffer: In, out_buffer: Out)
where
    In: Index<usize, Output=dyn Index<usize, Output=f32>>,
    Out: IndexMut<usize, Output=dyn IndexMut<usize, Output=f32>>
{
    /*
    for ((i0, i1), o0) in *in_buffer[0].zip(*in_buffer[1]).zip(*out_buffer[0]) {
        *o0 = i0 + i1;
    }
    */
}


fn main() {
    let buffer_size = 1024;

    let num_inputs = 2;
    let num_outputs = 2;

    let in_buffer = vec![vec![0f32; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0f32; buffer_size]; num_outputs];

    /*
    compute(
        buffer_size,
        in_buffer.iter().map(|buffer| buffer.as_slice()).collect::<Vec<&[f32]>>().as_slice(),
        out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [f32]>>().as_mut_slice(),
    );
    */

}