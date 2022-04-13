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

pub fn to_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: BinaryEncode + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    val.encode(&mut wr)?;
    Ok(wr)
}

pub trait BinaryEncode {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write;
}

// Fundamentals

impl BinaryEncode for bool {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        if *self {
            wr.write_all(&[1])?;
        } else {
            wr.write_all(&[0])?;
        }
        Ok(())
    }
}

impl BinaryEncode for i8 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i16 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for i64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u8 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u16 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for u64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for usize {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for f32 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for f64 {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        wr.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl BinaryEncode for str {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.len().encode(wr)?;
        wr.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: BinaryEncode> BinaryEncode for [T] {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.len().encode(wr)?;
        for x in self {
            x.encode(wr)?;
        }
        Ok(())
    }
}

impl<T: BinaryEncode> BinaryEncode for Option<T> {
    fn encode<W>(&self, wr: &mut W) -> Result<()>
    where
        W: Write,
    {
        match self {
            Some(x) => {
                wr.write_all(&[1])?;
                x.encode(wr)?;
            }
            None => {
                wr.write_all(&[0])?;
            }
        }
        Ok(())
    }
}

// Types

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integers() {
        let x: i8 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1]);
        let x: i16 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0]);
        let x: i32 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: i64 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
        let x: u8 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1]);
        let x: u16 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0]);
        let x: u32 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0]);
        let x: u64 = 1;
        assert_eq!(to_vec(&x).unwrap(), [1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_string() {
        let s = "hello";
        assert_eq!(
            to_vec(s).unwrap(),
            [5, 0, 0, 0, 0, 0, 0, 0, 104, 101, 108, 108, 111]
        );
        let s = "ä";
        assert_eq!(to_vec(s).unwrap(), [2, 0, 0, 0, 0, 0, 0, 0, 0xc3, 0xa4]);
    }

    #[test]
    fn test_array() {
        let a: &[i16] = &[1, 2, 3];
        assert_eq!(
            to_vec(a).unwrap(),
            [3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0]
        );
    }
}
