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
        self.s.encode(wr)?;
        self.d.encode(wr)?;
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
