use serde::Deserialize;
use serde::Serialize;

fn bool_is_false(x: &bool) -> bool {
    !(*x)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Props {
    #[serde(skip_serializing_if = "bool_is_false")]
    pub has_foo: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub has_bar: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub has_baz: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            has_foo: false,
            has_bar: false,
            has_baz: false,
        }
    }
}

#[cfg(test)]
mod test {
    use serde::de::DeserializeOwned;

    use super::*;

    fn roundtrip_serde_json<T: Clone + Serialize + DeserializeOwned>(value: T) -> T {
        let serialized = serde_json::to_vec(&value).unwrap();
        serde_json::from_slice(&serialized).unwrap()
    }

    fn roundtrip_rmp_serde<T: Clone + Serialize + DeserializeOwned>(value: T) -> T {
        // fails with compact/unnamed representation:
        // let serialized = rmp_serde::encode::to_vec(&value).unwrap();
        let serialized = rmp_serde::encode::to_vec_named(&value).unwrap();
        rmp_serde::decode::from_slice(&serialized).unwrap()
    }

    fn roundtrip_serde_bare<T: Clone + Serialize + DeserializeOwned>(value: T) -> T {
        let serialized = serde_bare::to_vec(&value).unwrap();
        serde_bare::from_slice(&serialized).unwrap()
    }

    fn roundtrip_bincode<T: Clone + Serialize + DeserializeOwned>(value: T) -> T {
        let serialized = bincode::serialize(&value).unwrap();
        bincode::deserialize(&serialized).unwrap()
    }

    #[test]
    fn test_serde_json() {
        let data_orig = Props {
            has_foo: false,
            has_bar: true,
            has_baz: false,
        };
        let data_reconstructed = roundtrip_serde_json(data_orig.clone());
        assert_eq!(data_reconstructed, data_orig);
    }

    #[test]
    fn test_rmp_serde() {
        let data_orig = Props {
            has_foo: false,
            has_bar: true,
            has_baz: false,
        };
        let data_reconstructed = roundtrip_rmp_serde(data_orig.clone());
        assert_eq!(data_reconstructed, data_orig);
    }

    #[test]
    #[ignore]
    fn test_serde_bare() {
        let data_orig = Props {
            has_foo: false,
            has_bar: true,
            has_baz: false,
        };
        let data_reconstructed = roundtrip_serde_bare(data_orig.clone());
        assert_eq!(data_reconstructed, data_orig);
    }

    #[test]
    #[ignore]
    fn test_bincode() {
        let data_orig = Props {
            has_foo: false,
            has_bar: true,
            has_baz: false,
        };
        let data_reconstructed = roundtrip_bincode(data_orig.clone());
        assert_eq!(data_reconstructed, data_orig);
    }
}
