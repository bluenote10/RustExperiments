use std::io::Result;
use std::io::Write;

pub fn to_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: BinaryEncode + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    val.encode(&mut wr)?;
    Ok(wr)
}

pub trait BinaryEncode {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write;
}
