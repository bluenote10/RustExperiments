use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub tempo_map: TempoMap,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TempoMap {
    pub bpm_base: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub name: String,
    pub is_percussion: bool,
    pub tuning: Tuning,
    pub notes: Vec<Note>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tuning {
    pub string_base_pitches: Vec<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub s: f64,
    pub d: f64,
    pub pitch: i32,
    pub string: i32,
    pub fret: i32,
    #[serde(default)]
    // as a field attribute, this means the individual field is initialized from Default::default() if it is missing.
    pub effects: NoteEffects,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // as a container attribute, this means that missing fields are taken from the struct's Default::default().
pub struct NoteEffects {
    pub dead_note: bool,
    pub vibrato: bool,
    pub bend_data: Option<BendData>,
}

impl Default for NoteEffects {
    fn default() -> Self {
        NoteEffects {
            dead_note: false,
            vibrato: false,
            bend_data: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BendData {
    pub points: Vec<BendPoint>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BendPoint {
    pub bend: f32,
    pub pos: f64,
}
