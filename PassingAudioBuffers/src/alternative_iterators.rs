fn compute<'a, I>(_num_to_generate: usize, in_buffer: &[&I], out_buffer: &mut[&I])
where
    I: Iterator<Item=&'a f32>
{
    /*
    for ((i0, i1), o0) in *in_buffer[0].zip(*in_buffer[1]).zip(*out_buffer[0]) {
        *o0 = i0 + i1;
    }
    */
}
