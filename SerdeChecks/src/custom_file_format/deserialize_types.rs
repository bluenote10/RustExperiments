use nom::number::complete::{le_f32, le_f64, le_i8, le_i32, le_u8};
use nom::IResult;

use crate::types::BendData;
use crate::types::BendPoint;
use crate::types::Note;
use crate::types::NoteEffects;
use crate::types::Sequence;
use crate::types::TempoMap;
use crate::types::Track;
use crate::types::Tuning;

use super::deserialize_fundamentals::{parse_bool, parse_option, parse_string, parse_vector};
use super::uint::parse_uint;

fn parse_sequence(input: &[u8]) -> IResult<&[u8], Sequence> {
    let (input, file_version) = le_i8(input)?;
    let (input, time_quantization) = parse_uint(input)?;
    let (input, tempo_map) = parse_tempo_map(input)?;
    let (input, tracks) = parse_vector(parse_track(time_quantization))(input)?;
    Ok((input, Sequence { tempo_map, tracks }))
}

fn parse_tempo_map(input: &[u8]) -> IResult<&[u8], TempoMap> {
    let (input, bpm_base) = le_f64(input)?;
    Ok((input, TempoMap { bpm_base }))
}

fn parse_track(time_quantization: u64) -> impl Fn(&[u8]) -> IResult<&[u8], Track> {
    move |input: &[u8]| {
        let (input, name) = parse_string(input)?;
        let (input, is_percussion) = parse_bool(input)?;
        let (input, tuning) = parse_tuning(input)?;
        let (input, notes) = parse_vector(parse_note(time_quantization))(input)?;
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

fn parse_note(time_quantization: u64) -> impl Fn(&[u8]) -> IResult<&[u8], Note> {
    move |input: &[u8]| {
        let (input, beat) = parse_uint(input)?;
        let (input, quantized_offset) = parse_uint(input)?;
        let (input, quantized_duration) = parse_uint(input)?;
        let (input, pitch) = le_u8(input)?;
        let (input, string) = le_u8(input)?;
        let (input, fret) = le_u8(input)?;
        let (input, effects) = parse_note_effects(input)?;
        let s = beat as f64 + quantized_offset as f64 * time_quantization as f64;
        let d = quantized_duration as f64 * time_quantization as f64;
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

fn parse_note_effects(input: &[u8]) -> IResult<&[u8], NoteEffects> {
    let (input, dead_note) = parse_bool(input)?;
    let (input, vibrato) = parse_bool(input)?;
    let (input, bend_data) = parse_option(parse_bend_data)(input)?;
    Ok((
        input,
        NoteEffects {
            dead_note,
            vibrato,
            bend_data,
        },
    ))
}

fn parse_bend_data(input: &[u8]) -> IResult<&[u8], BendData> {
    let (input, points) = parse_vector(parse_bend_point)(input)?;
    Ok((input, BendData { points }))
}

fn parse_bend_point(input: &[u8]) -> IResult<&[u8], BendPoint> {
    let (input, bend) = le_f32(input)?;
    let (input, pos) = le_f64(input)?;
    Ok((input, BendPoint { bend, pos }))
}
