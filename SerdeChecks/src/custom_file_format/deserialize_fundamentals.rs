use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::IResult;
use nom::Parser;

use super::uint::parse_uint;

/*
pub fn parse_vector<'a, O, F>(mut f: F) -> impl FnMut(&'a [u8]) -> IResult<&[u8], Vec<O>>
where
    //F: Parser<&'a [u8], O, Error<&'a [u8]>>,
    F: FnOnce(&'a [u8]) -> IResult<&[u8], O>,
{
    move |input: &'a [u8]| {
        //let mut input = i; //.clone();
        //let mut input = input.clone();

        let (input, num) = parse_uint(input)?;
        let (input, res) = count(f, num.0.try_into().unwrap())(input)?;

        Ok((input, res))
    }
}
*/

/*
pub fn parse_vector<'a, 'b, O, F>(mut f: F) -> impl Parser<&'b [u8], Vec<O>, Error<&'b [u8]>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
    //F: FnOnce(&'a [u8]) -> IResult<&[u8], O>,
{
    parse_uint.and_then(|num: Uint| count(f, num.0.try_into().unwrap()))
}
*/

pub fn parse_bool(input: &[u8]) -> IResult<&[u8], bool> {
    map(le_u8, |x| x != 0)(input)
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, size) = parse_uint(input)?;
    let (input, bytes) = take(size)(input)?;
    let res = String::from_utf8(bytes.to_owned()).unwrap(); // TODO: Map error
    Ok((input, res))
}

pub fn parse_vector<'a, O, F>(f: F, input: &'a [u8]) -> IResult<&[u8], Vec<O>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
{
    let (input, size) = parse_uint(input)?;
    let (input, res) = count(f, size.try_into().unwrap())(input)?; // TODO: Map error
    Ok((input, res))
}

pub fn parse_option<'a, O, F>(mut f: F, input: &'a [u8]) -> IResult<&[u8], Option<O>>
where
    F: Parser<&'a [u8], O, Error<&'a [u8]>>,
{
    let (input, is_defined) = parse_bool(input)?;
    if is_defined {
        let (input, res) = f.parse(input)?;
        Ok((input, Some(res)))
    } else {
        Ok((input, None))
    }
}
