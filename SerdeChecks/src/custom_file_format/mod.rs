mod deserialize_fundamentals;
mod deserialize_types;
mod serialize;
mod serialize_fundamentals;
mod serialize_types;
mod varint;

pub use deserialize_types::parse_sequence;
pub use serialize::serialize_into_vec;
pub use serialize_types::{serialize_sequence, serialize_sequence_to_vec};

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::types::BendData;
    use crate::types::BendPoint;
    use crate::types::Note;
    use crate::types::NoteEffects;
    use crate::types::Sequence;
    use crate::types::TempoMap;
    use crate::types::Track;
    use crate::types::Tuning;

    use super::*;

    const TIME_QUANTIZATION: i32 = 960;
    const PITCH_QUANTIZATION: i32 = 256;

    fn gen_random_string<R: Rng>(rng: &mut R) -> String {
        let len = rng.gen_range(0..=10);
        rng.sample_iter::<char, _>(rand::distributions::Standard)
            .take(len)
            .collect()
    }

    fn gen_sequence<R: Rng>(rng: &mut R) -> Sequence {
        Sequence {
            tempo_map: gen_tempo_map(rng),
            tracks: (0..rng.gen_range(0..=10)).map(|_| gen_track(rng)).collect(),
        }
    }

    fn gen_tempo_map<R: Rng>(rng: &mut R) -> TempoMap {
        TempoMap {
            bpm_base: rng.gen_range(20.0..=300.0),
        }
    }

    fn gen_track<R: Rng>(rng: &mut R) -> Track {
        Track {
            name: gen_random_string(rng),
            is_percussion: rng.gen_bool(0.5),
            tuning: gen_tuning(rng),
            notes: (0..rng.gen_range(0..=10)).map(|_| gen_note(rng)).collect(),
        }
    }

    fn gen_tuning<R: Rng>(rng: &mut R) -> Tuning {
        Tuning {
            string_base_pitches: (0..rng.gen_range(0..=10))
                .map(|_| rng.gen::<i32>())
                .collect(),
        }
    }

    fn gen_note<R: Rng>(rng: &mut R) -> Note {
        Note {
            s: rng.gen::<u16>() as f64
                + (rng.gen_range(0..TIME_QUANTIZATION) as f64 / TIME_QUANTIZATION as f64),
            d: rng.gen_range(0..TIME_QUANTIZATION * 10) as f64 / TIME_QUANTIZATION as f64,
            pitch: rng.gen::<u8>(),
            string: rng.gen::<u8>(),
            fret: rng.gen::<u8>(),
            effects: gen_note_effects(rng),
        }
    }

    fn gen_note_effects<R: Rng>(rng: &mut R) -> NoteEffects {
        NoteEffects {
            dead_note: rng.gen_bool(0.5),
            vibrato: rng.gen_bool(0.5),
            bend_data: if rng.gen_bool(0.5) {
                Some(gen_bend_data(rng))
            } else {
                None
            },
        }
    }

    fn gen_bend_data<R: Rng>(rng: &mut R) -> BendData {
        BendData {
            points: (0..rng.gen_range(0..=10))
                .map(|_| gen_bend_point(rng))
                .collect(),
        }
    }

    fn gen_bend_point<R: Rng>(rng: &mut R) -> BendPoint {
        BendPoint {
            pos: rng.gen_range(0..TIME_QUANTIZATION * 10) as f64 / TIME_QUANTIZATION as f64,
            bend: rng.gen_range(-PITCH_QUANTIZATION * 5..PITCH_QUANTIZATION * 5) as f32
                / PITCH_QUANTIZATION as f32,
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut rng = StdRng::seed_from_u64(0);
        let num_runs = 100;
        for _ in 0..num_runs {
            let sequence = gen_sequence(&mut rng);

            let serialized = serialize_sequence_to_vec(&sequence).unwrap();

            let (_, sequence_reconstructed) = parse_sequence(&serialized).unwrap();

            assert_eq!(sequence, sequence_reconstructed);
        }
    }
}
