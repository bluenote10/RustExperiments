
trait AudioBuffers<'a> {
    type T;
    fn get_buffer(&self, i: usize) -> &'a [Self::T];
}

trait AudioBuffersMut<'a> {
    type T;
    fn get_buffer(&self, i: usize) -> &'a mut [Self::T];
}

fn compute<'a, In, Out>(_num_to_generate: usize, in_buffer: In, out_buffer: Out)
where
    In: AudioBuffers<'a, T=f32>,
    Out: AudioBuffersMut<'a, T=f32>
{
    let input0 = in_buffer.get_buffer(0);
    let input1 = in_buffer.get_buffer(1);
    // Oh, this can't work due to multiple mutable borrows, right?
    let output0 = out_buffer.get_buffer(0);
    let output1 = out_buffer.get_buffer(1);
    /*
    for ((i0, i1), o0) in *in_buffer[0].zip(*in_buffer[1]).zip(*out_buffer[0]) {
        *o0 = i0 + i1;
    }
    */
}

/*
impl<'a, T> AudioBuffers<'a> for Vec<Vec<T>> {
    type T = T;
    fn get_buffer(&self, i: usize) -> &'a [Self::T] {
        &self[i]
    }
}
*/
