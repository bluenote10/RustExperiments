mod cereal_like;
mod custom_file_format;
mod semantics;
pub mod types;

pub use custom_file_format::parse_sequence;
pub use custom_file_format::{serialize_sequence, serialize_sequence_to_vec, Params};
