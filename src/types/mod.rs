use base64::Engine as _;
use serde::{Deserialize, Deserializer, Serializer};

pub mod agent_card;
pub mod message;
pub mod push;
pub mod requests;
pub mod responses;
pub mod security;
pub mod task;

pub use self::agent_card::*;
pub use self::message::*;
pub use self::push::*;
pub use self::requests::*;
pub use self::responses::*;
pub use self::security::*;
pub use self::task::*;

pub type JsonObject = serde_json::Map<String, serde_json::Value>;

pub(crate) fn is_false(value: &bool) -> bool {
    !*value
}

pub(crate) mod base64_bytes {
    use super::*;

    const ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

    pub(crate) mod option {
        use super::*;

        pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(bytes) => serializer.serialize_some(&ENGINE.encode(bytes)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let encoded = Option::<String>::deserialize(deserializer)?;
            encoded
                .map(|encoded| ENGINE.decode(encoded).map_err(serde::de::Error::custom))
                .transpose()
        }
    }
}
