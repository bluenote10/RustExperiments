use std::ops::RangeFrom;

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::IResult;
use nom::InputIter;
use nom::InputLength;
use nom::Parser;
use nom::Slice;

use super::uint::parse_uint;

pub fn parse_bool<I>(input: I) -> IResult<I, bool>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    map(le_u8, |x| x != 0)(input)
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, size) = parse_uint(input)?;
    let (input, bytes) = take(size)(input)?;
    let res = String::from_utf8(bytes.to_owned()).unwrap(); // TODO: Map error
    Ok((input, res))
}

pub fn parse_vector<'a, O, F>(mut f: F) -> impl FnMut(&'a [u8]) -> IResult<&[u8], Vec<O>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
{
    move |input: &'a [u8]| {
        let (input, size) = parse_uint(input)?;
        let (input, res) = count(|input| f.parse(input), size.try_into().unwrap())(input)?; // TODO: Map error
        Ok((input, res))
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
