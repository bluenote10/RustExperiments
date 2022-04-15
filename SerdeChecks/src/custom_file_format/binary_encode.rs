use std::io::Result;
use std::io::Write;

#[allow(dead_code)]
pub fn to_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: BinaryEncode<()> + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    let context = ();
    val.encode(&mut wr, &context)?;
    Ok(wr)
}

pub trait BinaryEncode<C> {
    fn encode<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write;
}
