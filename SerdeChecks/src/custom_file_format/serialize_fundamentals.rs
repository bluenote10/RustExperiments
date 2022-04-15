use std::io::Result;
use std::io::Write;

use super::serialize::Serialize;
use super::uint::Uint;

impl<C> Serialize<C> for bool {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        if *self {
            wr.write_all(&[1])?;
        } else {
            wr.write_all(&[0])?;
        }
        Ok(())
    }
}

impl<C> Serialize<C> for i8 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for i16 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for i32 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for i64 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for u8 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for u16 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for u32 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for u64 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for usize {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for f32 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for f64 {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl<C> Serialize<C> for str {
    fn serialize_into<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write,
    {
        Uint(self.len() as u64).serialize_into(wr, context)?;
        wr.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: Serialize<C>, C> Serialize<C> for [T] {
    fn serialize_into<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write,
    {
        Uint(self.len() as u64).serialize_into(wr, context)?;
        for x in self {
            x.serialize_into(wr, context)?;
        }
        Ok(())
    }
}

impl<T: Serialize<C>, C> Serialize<C> for Option<T> {
    fn serialize_into<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write,
    {
        match self {
            Some(x) => {
                wr.write_all(&[1])?;
                x.serialize_into(wr, context)?;
            }
            None => {
                wr.write_all(&[0])?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::serialize::serialize_into_vec;

    #[test]
    fn test_integers() {
        let x: i8 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1]);
        let x: i16 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0]);
        let x: i32 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: i64 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
        let x: u8 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1]);
        let x: u16 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0]);
        let x: u32 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: u64 = 1;
        assert_eq!(serialize_into_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_string() {
        let s = "hello";
        assert_eq!(serialize_into_vec(s).unwrap(), [5, 104, 101, 108, 108, 111]);
        let s = "ä";
        assert_eq!(serialize_into_vec(s).unwrap(), [2, 0xc3, 0xa4]);
    }

    #[test]
    fn test_array() {
        let a: &[i16] = &[1, 2, 3];
        assert_eq!(serialize_into_vec(a).unwrap(), [3, 1, 0, 2, 0, 3, 0]);
    }
}
