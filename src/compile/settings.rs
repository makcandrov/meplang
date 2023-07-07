use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompilerSettings {
    #[serde(default = "bool_true")]
    pub push0: bool,
    #[serde(default)]
    pub filling_pattern: FillingPatern,
}

impl Default for CompilerSettings {
    fn default() -> Self {
        Self { push0: true, filling_pattern: FillingPatern::default() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum FillingPatern {
    Random,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Repeat(Bytes),
}

impl Default for FillingPatern {
    fn default() -> Self {
        Self::Repeat(vec![0x00u8].into())
    }
}

pub fn serialize_bytes<S, T>(x: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    s.serialize_str(&format!("0x{}", hex::encode(x.as_ref())))
}

pub fn deserialize_bytes<'de, D>(d: D) -> Result<bytes::Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(d)?;
    if let Some(value) = value.strip_prefix("0x") {
        hex::decode(value)
    } else {
        hex::decode(&value)
    }
    .map(Into::into)
    .map_err(|e| serde::de::Error::custom(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_serialize() {
        let settings: CompilerSettings =
            serde_json::from_str("{\"push0\": true, \"fillingPattern\": {\"repeat\": \"0x11\"}}")
                .unwrap();
        dbg!(&settings);
    }
}

pub fn bool_true() -> bool {
    true
}
