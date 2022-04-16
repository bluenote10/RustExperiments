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

    let (input, _) = take_while(|c| {
        let i = i_cell.get();
        let x = x_cell.get();
        let is_last = i == 8;
        println!("{} {} {} {}", i, c, is_last, x);

        i_cell.set(i + 1);
        x_cell.set(x + ((if is_last { c } else { c & 0x7f } as u64) << (i as u64 * 7)));

        c >= 0x80 && !is_last
    })(input)?;

    Ok((input, Uint(x_cell.get())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_uint() {
        assert_eq!(parse_uint(&[0]).unwrap().1, Uint(0));
        assert_eq!(parse_uint(&[1]).unwrap().1, Uint(1));
        assert_eq!(parse_uint(&[0x7f]).unwrap().1, Uint(0x7f));

        assert_eq!(parse_uint(&[0x80, 0x01]).unwrap().1, Uint(0x80));
        assert_eq!(parse_uint(&[0xff, 0x01]).unwrap().1, Uint(0xff));

        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])
                .unwrap()
                .1,
            Uint(u64::MAX)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])
                .unwrap()
                .1,
            Uint(u64::MAX >> 8)
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f])
                .unwrap()
                .1,
            Uint(u64::MAX >> (8 + 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap().1,
            Uint(u64::MAX >> (8 + 2 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap().1,
            Uint(u64::MAX >> (8 + 3 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0xff, 0x7f]).unwrap().1,
            Uint(u64::MAX >> (8 + 4 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0xff, 0x7f]).unwrap().1,
            Uint(u64::MAX >> (8 + 5 * 7))
        );
        assert_eq!(
            parse_uint(&[0xff, 0x7f]).unwrap().1,
            Uint(u64::MAX >> (8 + 6 * 7))
        );
    }
}
