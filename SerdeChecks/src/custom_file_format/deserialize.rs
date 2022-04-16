use std::cell::Cell;

use nom::{bytes::complete::take_while, IResult};

use super::uint::Uint;

#[allow(dead_code)]
fn parse_uint(input: &[u8]) -> IResult<&[u8], Uint> {
    // let (input, _) = take(1usize)(input)?;
    // let mut x: u64 = 0;
    // let mut i = 0;

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
            println!("{} {} {} {}", i, c, is_last, x);

            i_cell.set(i + 1);
            x_cell.set(x + ((if is_last { c } else { c & 0x7f } as u64) << (i as u64 * 7)));

            end_reached_cell.set(c < 0x80 || is_last);
            true
        }
    })(input)?;

    Ok((input, Uint(x_cell.get())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_uint() {
        assert_eq!(parse_uint(&[0x00]).unwrap(), (&[][..], Uint(0x00)));
        assert_eq!(parse_uint(&[0x01]).unwrap(), (&[][..], Uint(0x01)));
        assert_eq!(parse_uint(&[0x7f]).unwrap(), (&[][..], Uint(0x7f)));

        assert_eq!(parse_uint(&[0x80, 0x01]).unwrap(), (&[][..], Uint(0x80)));
        assert_eq!(parse_uint(&[0xff, 0x01]).unwrap(), (&[][..], Uint(0xff)));

        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).unwrap(),
            (&[][..], Uint(u64::MAX))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> 8))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 2 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 3 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 4 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 5 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x7f]).unwrap(),
            (&[][..], Uint(u64::MAX >> (8 + 6 * 7)))
        );
    }

    #[test]
    fn test_parse_uint_continuation() {
        // all inputs here are followed up by a 0xff byte, which must be available in the output then.
        assert_eq!(
            parse_uint(&[0x00, 0xff]).unwrap(),
            (&[0xff][..], Uint(0x00))
        );
        assert_eq!(
            parse_uint(&[0x01, 0xff]).unwrap(),
            (&[0xff][..], Uint(0x01))
        );
        assert_eq!(
            parse_uint(&[0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(0x7f))
        );

        assert_eq!(
            parse_uint(&[0x80, 0x01, 0xff]).unwrap(),
            (&[0xff][..], Uint(0x80))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x01, 0xff]).unwrap(),
            (&[0xff][..], Uint(0xff))
        );

        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> 8))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 2 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 3 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 4 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 5 * 7)))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x7f, 0xff]).unwrap(),
            (&[0xff][..], Uint(u64::MAX >> (8 + 6 * 7)))
        );
    }
}
