use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub tempo_map: TempoMap,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TempoMap {
    pub bpm_base: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub name: String,
    pub is_percussion: bool,
    pub tuning: Tuning,
    pub notes: Vec<Note>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tuning {
    pub string_base_pitches: Vec<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub s: f64,
    pub d: f64,
    pub pitch: u8,
    pub string: u8,
    pub fret: u8,
    #[serde(default)]
    // as a field attribute, this means the individual field is initialized from Default::default() if it is missing.
    #[serde(skip_serializing_if = "is_default_note_effects")]
    pub effects: NoteEffects,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // as a container attribute, this means that missing fields are taken from the struct's Default::default().
pub struct NoteEffects {
    #[serde(skip_serializing_if = "bool_is_false")]
    pub dead_note: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub vibrato: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
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

fn bool_is_false(x: &bool) -> bool {
    !(*x)
}

fn is_default_note_effects(note_effects: &NoteEffects) -> bool {
    *note_effects == NoteEffects::default()
}
