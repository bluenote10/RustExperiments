use std::ops::RangeFrom;

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::{Error, ErrorKind};
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::Err;
use nom::{IResult, InputIter, InputLength, Parser, Slice};

use super::uint::parse_uint;

pub fn parse_bool<I>(input: I) -> IResult<I, bool>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    map(le_u8, |x| x != 0)(input)
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, num_bytes) = parse_uint(input)?;
    if num_bytes as usize > input.len() {
        Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))
    } else {
        let (input, bytes) = take(num_bytes)(input)?;
        let res = String::from_utf8(bytes.to_owned())
            .map_err(|_| Err::Error(Error::new(input, ErrorKind::Fail)))?; // for lack of more fitting error kind
        Ok((input, res))
    }
}

pub fn parse_vector<'a, O, F>(mut f: F) -> impl FnMut(&'a [u8]) -> IResult<&[u8], Vec<O>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
{
    move |input: &'a [u8]| {
        let (input, num_elements) = parse_uint(input)?;
        // Assuming an element has a minimum size of 1, we need at least that many bytes.
        if num_elements as usize > input.len() {
            Err(Err::Error(Error::new(input, ErrorKind::TooLarge)))
        } else {
            let size: usize = num_elements
                .try_into()
                .map_err(|_| Err::Error(Error::new(input, ErrorKind::TooLarge)))?;
            let (input, res) = count(|input| f.parse(input), size)(input)?;
            Ok((input, res))
        }
    }
}

pub fn parse_option<'a, O, F>(mut f: F) -> impl FnMut(&'a [u8]) -> IResult<&[u8], Option<O>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
{
    move |input: &'a [u8]| {
        let (input, is_defined) = parse_bool(input)?;
        if is_defined {
            let (input, res) = f.parse(input)?;
            Ok((input, Some(res)))
        } else {
            Ok((input, None))
        }
    }
}
