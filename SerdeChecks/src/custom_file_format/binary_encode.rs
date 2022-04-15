use std::io::Result;
use std::io::Write;

pub fn to_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: BinaryEncode + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    let context = ();
    val.encode(&mut wr, &context)?;
    Ok(wr)
}

pub trait BinaryEncode {
    fn encode<W, C>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write;
}
