use std::cell::Cell;
use std::io::Result;
use std::io::Write;

use nom::{bytes::complete::take_while, IResult};

use super::serialize::Serialize;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Uint(pub u64);

impl<C> Serialize<C> for Uint {
    fn serialize_into<W>(&self, wr: &mut W, _context: &C) -> Result<()>
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

pub fn parse_uint(input: &[u8]) -> IResult<&[u8], u64> {
    let x_cell = Cell::<u64>::new(0);
    let i_cell = Cell::new(0);
    let end_reached_cell = Cell::new(false);

    let (input, _) = take_while(|c| {
        let end_reached = end_reached_cell.get();
        if end_reached {
            false
        } else {
            let i = i_cell.get();
            let x = x_cell.get();
            let is_last = i == 8;

            i_cell.set(i + 1);
            x_cell.set(x + ((if is_last { c } else { c & 0x7f } as u64) << (i as u64 * 7)));

            end_reached_cell.set(c < 0x80 || is_last);
            true
        }
    })(input)?;

    Ok((input, x_cell.get()))
}

#[cfg(test)]
mod test {
    use super::super::serialize::serialize_into_vec;
    use super::*;

    #[test]
    fn test_uint_serialization() {
        let x = Uint(0);
        assert_eq!(serialize_into_vec(&x).unwrap(), [0]);
        let x = Uint(1);
        assert_eq!(serialize_into_vec(&x).unwrap(), [1]);
        let x = Uint(0x7f);
        assert_eq!(serialize_into_vec(&x).unwrap(), [0x7f]);
        let x = Uint(0x80);
        assert_eq!(serialize_into_vec(&x).unwrap(), [0x80, 0x01]);
        let x = Uint(0xff);
        assert_eq!(serialize_into_vec(&x).unwrap(), [0xff, 0x01]);

        let x = Uint(u64::MAX);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        let x = Uint(u64::MAX - 1);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );

        let x = Uint(u64::MAX >> 1);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> 2);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3f]
        );
        let x = Uint(u64::MAX >> 3);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f]
        );
        let x = Uint(u64::MAX >> 4);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f]
        );

        let x = Uint(u64::MAX >> 5);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x07]
        );
        let x = Uint(u64::MAX >> 6);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03]
        );
        let x = Uint(u64::MAX >> 7);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
        let x = Uint(u64::MAX >> 8);
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );

        let x = Uint(u64::MAX >> (8 + 7));
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> (8 + 2 * 7));
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> (8 + 3 * 7));
        assert_eq!(
            serialize_into_vec(&x).unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        let x = Uint(u64::MAX >> (8 + 4 * 7));
        assert_eq!(serialize_into_vec(&x).unwrap(), [0xff, 0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 5 * 7));
        assert_eq!(serialize_into_vec(&x).unwrap(), [0xff, 0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 6 * 7));
        assert_eq!(serialize_into_vec(&x).unwrap(), [0xff, 0x7f]);
        let x = Uint(u64::MAX >> (8 + 7 * 7));
        assert_eq!(serialize_into_vec(&x).unwrap(), [0x7f]);
    }

    #[test]
    fn test_parse_uint() {
        assert_eq!(parse_uint(&[0x00]).unwrap(), (&[][..], 0x00));
        assert_eq!(parse_uint(&[0x01]).unwrap(), (&[][..], 0x01));
        assert_eq!(parse_uint(&[0x7f]).unwrap(), (&[][..], 0x7f));

        assert_eq!(parse_uint(&[0x80, 0x01]).unwrap(), (&[][..], 0x80));
        assert_eq!(parse_uint(&[0xff, 0x01]).unwrap(), (&[][..], 0xff));

        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).unwrap(),
            (&[][..], u64::MAX)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> 8)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 2 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 3 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 4 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 5 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x7f]).unwrap(),
            (&[][..], u64::MAX >> (8 + 6 * 7))
        );
    }

    #[test]
    fn test_parse_uint_continuation() {
        // all inputs here are followed up by a 0xff byte, which must be available in the output then.
        assert_eq!(parse_uint(&[0x00, 0xff]).unwrap(), (&[0xff][..], 0x00));
        assert_eq!(parse_uint(&[0x01, 0xff]).unwrap(), (&[0xff][..], 0x01));
        assert_eq!(parse_uint(&[0x7f, 0xff]).unwrap(), (&[0xff][..], 0x7f));

        assert_eq!(
            parse_uint(&[0x80, 0x01, 0xff]).unwrap(),
            (&[0xff][..], 0x80)
        );
        assert_eq!(
            parse_uint(&[0xff, 0x01, 0xff]).unwrap(),
            (&[0xff][..], 0xff)
        );

        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> 8)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 2 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 3 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 4 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 5 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], u64::MAX >> (8 + 6 * 7))
        );
    }
}
