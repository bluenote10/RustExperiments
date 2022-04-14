use std::io::Result;
use std::io::Write;

use super::binary_encode::BinaryEncode;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Uint(pub u64);

impl BinaryEncode for Uint {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        let mut x = self.0;
        let mut buf = [0u8; 10];
        let mut i = 0usize;

        loop {
            let is_last = x < 0x80 || i == 8;
            if !is_last {
                buf[i] = (x as u8) | 0x80;
            } else {
                buf[i] = x as u8;
            }
            x >>= 7;
            i += 1;
            if is_last {
                break;
            }
        }

        wr.write_all(&buf[..i])?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::binary_encode::to_vec;
    use super::*;

    #[test]
    fn test_encode() {
        let x = Uint(0);
        assert_eq!(to_vec(&x).unwrap(), [0]);
        let x = Uint(1);
        assert_eq!(to_vec(&x).unwrap(), [1]);
        let x = Uint(0x7f);
        assert_eq!(to_vec(&x).unwrap(), [0x7f]);
        let x = Uint(0x80);
        assert_eq!(to_vec(&x).unwrap(), [0x80, 0x01]);
        let x = Uint(0xff);
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0x01]);

        let x = Uint(u64::MAX);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        let x = Uint(u64::MAX - 1);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );

        let x = Uint(u64::MAX >> 1);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> 2);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3f]
        );
        let x = Uint(u64::MAX >> 3);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f]
        );
        let x = Uint(u64::MAX >> 4);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f]
        );

        let x = Uint(u64::MAX >> 5);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x07]
        );
        let x = Uint(u64::MAX >> 6);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03]
        );
        let x = Uint(u64::MAX >> 7);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
        let x = Uint(u64::MAX >> 8);
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );

        let x = Uint(u64::MAX >> (8 + 7));
        assert_eq!(
            to_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> (8 + 2 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 3 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0xff, 0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 4 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 5 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 6 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 7 * 7));
        assert_eq!(to_vec(&x).unwrap(), [0x7f]);
    }
}
