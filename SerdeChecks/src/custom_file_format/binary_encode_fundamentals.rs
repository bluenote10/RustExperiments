use std::io::Result;
use std::io::Write;

use super::binary_encode::BinaryEncode;

impl BinaryEncode for bool {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
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

impl BinaryEncode for i8 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i16 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u8 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u16 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for usize {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for f32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for f64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for str {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.len().encode(wr)?;
        wr.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: BinaryEncode> BinaryEncode for [T] {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.len().encode(wr)?;
        for x in self {
            x.encode(wr)?;
        }
        Ok(())
    }
}

impl<T: BinaryEncode> BinaryEncode for Option<T> {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        match self {
            Some(x) => {
                wr.write_all(&[1])?;
                x.encode(wr)?;
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
    use super::super::binary_encode::to_vec;

    #[test]
    fn test_integers() {
        let x: i8 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1]);
        let x: i16 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0]);
        let x: i32 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: i64 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
        let x: u8 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1]);
        let x: u16 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0]);
        let x: u32 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: u64 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_string() {
        let s = "hello";
        assert_eq!(
            to_vec(s).unwrap(),
            [5, 0, 0, 0, 0, 0, 0, 0, 104, 101, 108, 108, 111]
        );
        let s = "ä";
        assert_eq!(to_vec(s).unwrap(), [2, 0, 0, 0, 0, 0, 0, 0, 0xc3, 0xa4]);
    }

    #[test]
    fn test_array() {
        let a: &[i16] = &[1, 2, 3];
        assert_eq!(
            to_vec(a).unwrap(),
            [3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0]
        );
    }
}
