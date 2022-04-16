use std::io::Result;
use std::io::Write;

use crate::types::BendData;
use crate::types::BendPoint;
use crate::types::Note;
use crate::types::NoteEffects;
use crate::types::Sequence;
use crate::types::TempoMap;
use crate::types::Track;
use crate::types::Tuning;

use super::serialize::Serialize;
use super::uint::Uint;

struct Params {
    time_quantization: u64,
}

pub fn serialize_sequence<W>(sequence: &Sequence, mut wr: W) -> Result<()>
where
    W: Write,
{
    let context = Params {
        time_quantization: 960,
    };
    sequence.serialize_into(&mut wr, &context)
}

impl Serialize<Params> for Sequence {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        let file_version: i8 = 0;
        file_version.serialize_into(wr, context)?;
        Uint(context.time_quantization).serialize_into(wr, context)?;
        self.tempo_map.serialize_into(wr, context)?;
        self.tracks.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for TempoMap {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.bpm_base.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for Track {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.name.serialize_into(wr, context)?;
        self.is_percussion.serialize_into(wr, context)?;
        self.tuning.serialize_into(wr, context)?;
        self.notes.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for Tuning {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.string_base_pitches.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for Note {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        let (beat, quantized_offset) = split_in_beat_and_offset(self.s, context.time_quantization);
        let quantized_duration = quantize_duration(self.d, context.time_quantization);
        beat.serialize_into(wr, context)?;
        quantized_offset.serialize_into(wr, context)?;
        quantized_duration.serialize_into(wr, context)?;
        // self.s.encode(wr)?;
        // self.d.encode(wr)?;
        self.pitch.serialize_into(wr, context)?;
        self.string.serialize_into(wr, context)?;
        self.fret.serialize_into(wr, context)?;
        self.effects.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for NoteEffects {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.dead_note.serialize_into(wr, context)?;
        self.vibrato.serialize_into(wr, context)?;
        self.bend_data.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for BendData {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.points.serialize_into(wr, context)?;
        Ok(())
    }
}

impl Serialize<Params> for BendPoint {
    fn serialize_into<W>(&self, wr: &mut W, context: &Params) -> Result<()>
    where
        W: Write,
    {
        self.bend.serialize_into(wr, context)?;
        self.pos.serialize_into(wr, context)?;
        Ok(())
    }
}

fn split_in_beat_and_offset(t: f64, time_quantization: u64) -> (Uint, Uint) {
    let beat = Uint(t as u64);
    let offset = quantize_duration(t - (beat.0 as f64), time_quantization);
    (beat, offset)
}

fn quantize_duration(duration: f64, time_quantization: u64) -> Uint {
    Uint((duration * time_quantization as f64) as u64)
}
