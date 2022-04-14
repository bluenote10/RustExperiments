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

impl BinaryEncode for Sequence {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        let file_version: i8 = 0;
        file_version.encode(wr)?;
        self.tempo_map.encode(wr)?;
        self.tracks.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for TempoMap {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.bpm_base.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for Track {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.name.encode(wr)?;
        self.is_percussion.encode(wr)?;
        self.tuning.encode(wr)?;
        self.notes.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for Tuning {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.string_base_pitches.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for Note {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        let (beat, quantized_offset) = split_in_beat_and_offset(self.s);
        let quantized_duration = quantize_duration(self.d);
        beat.encode(wr)?;
        quantized_offset.encode(wr)?;
        quantized_duration.encode(wr)?;
        // self.s.encode(wr)?;
        // self.d.encode(wr)?;
        self.pitch.encode(wr)?;
        self.string.encode(wr)?;
        self.fret.encode(wr)?;
        self.effects.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for NoteEffects {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.dead_note.encode(wr)?;
        self.vibrato.encode(wr)?;
        self.bend_data.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for BendData {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.points.encode(wr)?;
        Ok(())
    }
}

impl BinaryEncode for BendPoint {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.bend.encode(wr)?;
        self.pos.encode(wr)?;
        Ok(())
    }
}

fn split_in_beat_and_offset(t: f64) -> (Uint, Uint) {
    let beat = Uint(t as u64);
    let offset = quantize_duration(t - (beat.0 as f64));
    (beat, offset)
}

fn quantize_duration(duration: f64) -> Uint {
    Uint((duration * 960.0) as u64)
}
