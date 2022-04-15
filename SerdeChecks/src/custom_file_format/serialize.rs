use std::io::Result;
use std::io::Write;

#[allow(dead_code)]
pub fn serialize_into_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: Serialize<()> + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    let context = ();
    val.serialize_into(&mut wr, &context)?;
    Ok(wr)
}

pub trait Serialize<C> {
    fn serialize_into<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write;
}
