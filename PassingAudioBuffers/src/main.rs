mod alternative_iterators;
mod alternative_index_trait;
mod alternative_custom_trait;

fn compute(_num_to_generate: usize, _in_buffer: &[&[f32]], _out_buffer: &mut[&mut[f32]]) {
    // ...
}

fn main() {
    let buffer_size = 1024;

    let num_inputs = 2;
    let num_outputs = 2;

    let in_buffer = vec![vec![0f32; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0f32; buffer_size]; num_outputs];

    // Option 1: Dynamically allocate a vector of slices
    compute(
        buffer_size,
        in_buffer.iter().map(|buffer| buffer.as_slice()).collect::<Vec<&[f32]>>().as_slice(),
        out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [f32]>>().as_mut_slice(),
    );

    // When the sizes are known, it is possible to avoid the vector.
    // But that is only straightforward for the in_buffer. Applying the same pattern
    // for the out_buffer does not work, because of single mutable borrow rule.
    compute(
        buffer_size,
        &[&in_buffer[0], &in_buffer[1]],
        //&mut [&mut out_buffer[0], &mut out_buffer[1]], // => "cannot borrow `out_buffer` as mutable more than once at a time"
        out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [f32]>>().as_mut_slice(),
    );

    // To achieve the same for the out_buffer, some pattern matching is required
    {
        if let [out_buffer0, out_buffer1] = &mut out_buffer[0..2] {
            compute(
                buffer_size,
                &[&in_buffer[0], &in_buffer[1]],
                &mut [out_buffer0, out_buffer1],
            );
        }
    }

    // To make that more symmetric:
    {
        if let ([in_buffer0, in_buffer1], [out_buffer0, out_buffer1]) = (&in_buffer[0..2], &mut out_buffer[0..2]) {
            compute(
                buffer_size,
                &[in_buffer0, in_buffer1],
                &mut [out_buffer0, out_buffer1],
            );
        }
    }

    // Now let's attempt to use an expression that abstracts over multiple static sizes
    /*
    // Not possible because temporary doesn't live long enough
    compute(
        buffer_size,
        if num_inputs == 1 {
            &[&in_buffer[0]]
        } else if num_inputs == 2 {
            &[&in_buffer[0], &in_buffer[0]]
        } else {
            &[]
        },
        out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [f32]>>().as_mut_slice(),
    );
    */
    /*
    // Not possible because branches have incompatible type
    compute(
        buffer_size,
        {
            let buffer = if num_inputs == 1 {
                &[&in_buffer[0]]
            } else if num_inputs == 2 {
                &[&in_buffer[0], &in_buffer[0]]
            } else {
                &[]
            };
            buffer
        },
        out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [f32]>>().as_mut_slice(),
    );
    */
    // Is it really necessary to use an expression around the entire `compute` requiring to factorize num inputs x num outputs?
    {
        if let ([in_buffer0], [out_buffer0]) = (in_buffer.as_slice(), out_buffer.as_mut_slice()) {
            compute(
                buffer_size,
                &[in_buffer0],
                &mut [out_buffer0],
            );
        } if let ([in_buffer0, in_buffer1], [out_buffer0]) = (in_buffer.as_slice(), out_buffer.as_mut_slice()) {
            compute(
                buffer_size,
                &[in_buffer0, in_buffer1],
                &mut [out_buffer0],
            );
        } if let ([in_buffer0], [out_buffer0, out_buffer1]) = (in_buffer.as_slice(), out_buffer.as_mut_slice()) {
            compute(
                buffer_size,
                &[in_buffer0],
                &mut [out_buffer0, out_buffer1],
            );
        } if let ([in_buffer0, in_buffer1], [out_buffer0, out_buffer1]) = (in_buffer.as_slice(), out_buffer.as_mut_slice()) {
            compute(
                buffer_size,
                &[in_buffer0, in_buffer1],
                &mut [out_buffer0, out_buffer1],
            );
        } else {
            // fallback
        }
    }

    // Basically the problem is that it is not possible to write an expression for:
    /*
    let buffer = if num_inputs == 1 {
        &[&in_buffer[0]].as_ptr()
    } else {
        &[&in_buffer[0], &in_buffer[0]].as_ptr()
    };
    */
    /*
    let in_buffers_slices = if let [in_buffer0] = in_buffer.as_slice() {
        &[in_buffer0.as_slice()] as &[&[f32]]
    } else if let [in_buffer0, in_buffer1] = in_buffer.as_slice() {
        &[in_buffer0.as_slice(), in_buffer1.as_slice()] as &[&[f32]]
    } else {
        panic!("here")
    };
    */

    // Maybe with macro + nesting it would be possible to...
    if let [in_buffer0] = in_buffer.as_slice() {
        if let [out_buffer0] = out_buffer.as_mut_slice() {
            compute(
                buffer_size,
                &[in_buffer0],
                &mut [out_buffer0],
            );
        } else if let [out_buffer0, out_buffer1] = out_buffer.as_mut_slice() {
            compute(
                buffer_size,
                &[in_buffer0],
                &mut [out_buffer0, out_buffer1],
            );
        }
    } else if let [in_buffer0, in_buffer1] = in_buffer.as_slice() {
        if let [out_buffer0] = out_buffer.as_mut_slice() {
            compute(
                buffer_size,
                &[in_buffer0, in_buffer1],
                &mut [out_buffer0],
            );
        } else if let [out_buffer0, out_buffer1] = out_buffer.as_mut_slice() {
            compute(
                buffer_size,
                &[in_buffer0, in_buffer1],
                &mut [out_buffer0, out_buffer1],
            );
        }
    }

    // Maybe some of the "allocate vec on stack" approaches could solve it?
    // - https://stackoverflow.com/questions/29239586/how-can-i-create-a-stack-allocated-vector-like-container
    // - http://troubles.md/improving-smallvec/
    // - https://llogiq.github.io/2018/09/13/smallvec.html
    // - https://www.reddit.com/r/rust/comments/fshuhk/introducing_tinyvec_100_safe_alternative_to/

}