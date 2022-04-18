use nom::error::{Error, ErrorKind};
use nom::number::complete::{le_f64, le_i8, le_i32, le_u8};
use nom::{Err, IResult};

use crate::types::BendData;
use crate::types::BendPoint;
use crate::types::Note;
use crate::types::NoteEffects;
use crate::types::Sequence;
use crate::types::TempoMap;
use crate::types::Track;
use crate::types::Tuning;

use super::deserialize_fundamentals::{parse_bool, parse_option, parse_string, parse_vector};
use super::varint::{parse_int, parse_uint};

// Using a type alias for the verbose return type is impossible?
// https://stackoverflow.com/questions/53916203/alias-a-generic-function-with-lifetimes
// https://stackoverflow.com/a/26071172/1804173
// trait DefaultParser<T>: Fn(&[u8]) -> IResult<&[u8], T> {}
// impl<T, F: Fn(&[u8]) -> IResult<&[u8], T>> DefaultParser<T> for F {}
// Actually the following would work, but then the lifetime parameter has
// to be templated everywhere, making it just as verbose.
// trait DefaultParser<'a, T>: Fn(&'a [u8]) -> IResult<&'a [u8], T> {}
// impl<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>> DefaultParser<'a, T> for F {}

pub fn parse_sequence(input: &[u8]) -> IResult<&[u8], Sequence> {
    let (input, file_version) = le_i8(input)?;
    if file_version != 0 {
        return Err(Err::Error(Error::new(input, ErrorKind::Fail))); // for lack of more fitting error kind
    }
    let (input, time_quantization) = parse_uint(input)?;
    let (input, pitch_quantization) = parse_uint(input)?;
    let (input, tempo_map) = parse_tempo_map(input)?;
    let (input, tracks) = parse_vector(parse_track(time_quantization, pitch_quantization))(input)?;
    Ok((input, Sequence { tempo_map, tracks }))
}

fn parse_tempo_map(input: &[u8]) -> IResult<&[u8], TempoMap> {
    let (input, bpm_base) = le_f64(input)?;
    Ok((input, TempoMap { bpm_base }))
}

fn parse_track(
    time_quantization: u64,
    pitch_quantization: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], Track> {
    move |input: &[u8]| {
        let (input, name) = parse_string(input)?;
        let (input, is_percussion) = parse_bool(input)?;
        let (input, tuning) = parse_tuning(input)?;
        let (input, notes) =
            parse_vector(parse_note(time_quantization, pitch_quantization))(input)?;
        Ok((
            input,
            Track {
                name,
                is_percussion,
                tuning,
                notes,
            },
        ))
    }
}

fn parse_tuning(input: &[u8]) -> IResult<&[u8], Tuning> {
    let (input, string_base_pitches) = parse_vector(le_i32)(input)?;
    Ok((
        input,
        Tuning {
            string_base_pitches,
        },
    ))
}

fn parse_note(
    time_quantization: u64,
    pitch_quantization: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], Note> {
    move |input: &[u8]| {
        let (input, beat) = parse_uint(input)?;
        let (input, quantized_offset) = parse_uint(input)?;
        let (input, quantized_duration) = parse_uint(input)?;
        let (input, pitch) = le_u8(input)?;
        let (input, string) = le_u8(input)?;
        let (input, fret) = le_u8(input)?;
        let (input, effects) = parse_note_effects(time_quantization, pitch_quantization)(input)?;
        let s = beat as f64 + quantized_offset as f64 / time_quantization as f64;
        let d = quantized_duration as f64 / time_quantization as f64;
        Ok((
            input,
            Note {
                s,
                d,
                pitch,
                string,
                fret,
                effects,
            },
        ))
    }
}

fn parse_note_effects(
    time_quantization: u64,
    pitch_quantization: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], NoteEffects> {
    move |input: &[u8]| {
        let (input, dead_note) = parse_bool(input)?;
        let (input, vibrato) = parse_bool(input)?;
        let (input, bend_data) =
            parse_option(parse_bend_data(time_quantization, pitch_quantization))(input)?;
        Ok((
            input,
            NoteEffects {
                dead_note,
                vibrato,
                bend_data,
            },
        ))
    }
}

fn parse_bend_data(
    time_quantization: u64,
    pitch_quantization: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], BendData> {
    move |input: &[u8]| {
        let (input, points) =
            parse_vector(parse_bend_point(time_quantization, pitch_quantization))(input)?;
        Ok((input, BendData { points }))
    }
}

fn parse_bend_point(
    time_quantization: u64,
    pitch_quantization: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], BendPoint> {
    move |input: &[u8]| {
        let (input, quantized_pos) = parse_uint(input)?;
        let (input, quantized_bend) = parse_int(input)?;
        let pos = quantized_pos as f64 / time_quantization as f64;
        let bend = quantized_bend as f32 / pitch_quantization as f32;
        Ok((input, BendPoint { pos, bend }))
    }
}
