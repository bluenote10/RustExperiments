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

    fn get_random_string<R: Rng>(rnd: &mut R) -> String {
        let len = rnd.gen_range(0..=10);
        rnd.sample_iter::<char, _>(rand::distributions::Standard)
            .take(len)
            .collect()
    }

    fn gen_sequence<R: Rng>(rnd: &mut R) -> Sequence {
        Sequence {
            tempo_map: gen_tempo_map(rnd),
            tracks: (0..rnd.gen_range(0..=10)).map(|_| gen_track(rnd)).collect(),
        }
    }

    fn gen_tempo_map<R: Rng>(rnd: &mut R) -> TempoMap {
        TempoMap {
            bpm_base: rnd.gen_range(20.0..=300.0),
        }
    }

    fn gen_track<R: Rng>(rnd: &mut R) -> Track {
        Track {
            name: get_random_string(rnd),
            is_percussion: rnd.gen_bool(0.5),
            tuning: gen_tuning(rnd),
            notes: (0..rnd.gen_range(0..=10)).map(|_| gen_note(rnd)).collect(),
        }
    }

    fn gen_tuning<R: Rng>(rnd: &mut R) -> Tuning {
        Tuning {
            string_base_pitches: (0..rnd.gen_range(0..=10))
                .map(|_| rnd.gen::<i32>())
                .collect(),
        }
    }

    fn gen_note<R: Rng>(rnd: &mut R) -> Note {
        Note {
            s: rnd.gen::<u16>() as f64 + (rnd.gen_range(0..960) as f64 / 960.0),
            d: rnd.gen_range(0..960) as f64 / 960.0,
            pitch: rnd.gen::<u8>(),
            string: rnd.gen::<u8>(),
            fret: rnd.gen::<u8>(),
            effects: gen_note_effects(rnd),
        }
    }

    fn gen_note_effects<R: Rng>(rnd: &mut R) -> NoteEffects {
        NoteEffects {
            dead_note: rnd.gen_bool(0.5),
            vibrato: rnd.gen_bool(0.5),
            bend_data: if rnd.gen_bool(0.5) {
                Some(gen_bend_data(rnd))
            } else {
                None
            },
        }
    }

    fn gen_bend_data<R: Rng>(rnd: &mut R) -> BendData {
        BendData {
            points: (0..rnd.gen_range(0..=10))
                .map(|_| gen_bend_point(rnd))
                .collect(),
        }
    }

    fn gen_bend_point<R: Rng>(rnd: &mut R) -> BendPoint {
        BendPoint {
            bend: rnd.gen_range(-5.0..=5.0),
            pos: rnd.gen_range(0.0..=10.0),
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
