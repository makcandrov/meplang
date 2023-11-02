use std::collections::HashMap;

use bytes::Bytes;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompilerSettings {
    #[serde(default = "bool_true")]
    pub push0: bool,
    #[serde(default)]
    pub filling_pattern: FillingPatern,
    #[serde(default, serialize_with = "serialize_variables", deserialize_with = "deserialize_variables")]
    pub variables: HashMap<String, Bytes>,
}

impl Default for CompilerSettings {
    fn default() -> Self {
        Self {
            push0: true,
            filling_pattern: FillingPatern::default(),
            variables: HashMap::default(),
        }
    }
}

impl CompilerSettings {
    pub fn add_variable(&mut self, name: &str, value: Bytes) {
        self.variables.insert(name.to_owned(), value);
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

pub fn deserialize_bytes<'de, D>(d: D) -> Result<Bytes, D::Error>
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

pub fn serialize_variables<S>(x: &HashMap<String, Bytes>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = s.serialize_map(Some(x.len()))?;
    for (k, v) in x.iter() {
        map.serialize_entry(k, &format!("0x{}", hex::encode(v)))?;
    }
    map.end()
}

pub fn deserialize_variables<'de, D>(d: D) -> Result<HashMap<String, Bytes>, D::Error>
where
    D: Deserializer<'de>,
{
    let des: HashMap<String, String> = Deserialize::deserialize(d)?;
    let mut res = HashMap::<String, Bytes>::with_capacity(des.len());
    for (k, v) in des {
        let value = if let Some(v) = v.strip_prefix("0x") {
            hex::decode(v)
        } else {
            hex::decode(&v)
        }
        .map(Into::into)
        .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        res.insert(k, value);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_serialize() {
        let settings: CompilerSettings =
            serde_json::from_str("{\"push0\": true, \"fillingPattern\": {\"repeat\": \"0x11\"}}").unwrap();
        dbg!(&settings);
    }
}

pub fn bool_true() -> bool {
    true
}
