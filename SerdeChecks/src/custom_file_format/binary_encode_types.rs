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

use super::binary_encode::BinaryEncode;
use super::uint::Uint;

struct EncodeParams {
    time_quantization: u32,
}

pub fn serialize_sequence<W>(sequence: &Sequence, mut wr: W) -> Result<()>
where
    W: Write,
{
    let context = EncodeParams {
        time_quantization: 960,
    };
    sequence.encode(&mut wr, &context)
}

impl BinaryEncode<EncodeParams> for Sequence {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        let file_version: i8 = 0;
        Uint(context.time_quantization.into()).encode(wr, context)?;
        file_version.encode(wr, context)?;
        self.tempo_map.encode(wr, context)?;
        self.tracks.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for TempoMap {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.bpm_base.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for Track {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.name.encode(wr, context)?;
        self.is_percussion.encode(wr, context)?;
        self.tuning.encode(wr, context)?;
        self.notes.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for Tuning {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.string_base_pitches.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for Note {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        let (beat, quantized_offset) = split_in_beat_and_offset(self.s, context.time_quantization);
        let quantized_duration = quantize_duration(self.d, context.time_quantization);
        beat.encode(wr, context)?;
        quantized_offset.encode(wr, context)?;
        quantized_duration.encode(wr, context)?;
        // self.s.encode(wr)?;
        // self.d.encode(wr)?;
        self.pitch.encode(wr, context)?;
        self.string.encode(wr, context)?;
        self.fret.encode(wr, context)?;
        self.effects.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for NoteEffects {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.dead_note.encode(wr, context)?;
        self.vibrato.encode(wr, context)?;
        self.bend_data.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for BendData {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.points.encode(wr, context)?;
        Ok(())
    }
}

impl BinaryEncode<EncodeParams> for BendPoint {
    fn encode<W>(&self, wr: &mut W, context: &EncodeParams) -> Result<()>
    where
        W: Write,
    {
        self.bend.encode(wr, context)?;
        self.pos.encode(wr, context)?;
        Ok(())
    }
}

fn split_in_beat_and_offset(t: f64, time_quantization: u32) -> (Uint, Uint) {
    let beat = Uint(t as u64);
    let offset = quantize_duration(t - (beat.0 as f64), time_quantization);
    (beat, offset)
}

fn quantize_duration(duration: f64, time_quantization: u32) -> Uint {
    Uint((duration * time_quantization as f64) as u64)
}
