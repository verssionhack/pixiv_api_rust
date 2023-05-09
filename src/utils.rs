use chrono::NaiveDateTime;
use serde::{Deserializer, Deserialize, Serializer};

pub fn u64_deserializer<'de, D>(de: D) -> Result<u64, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(de)?;
    Ok(s.parse::<u64>().unwrap())
}
pub fn datetime_deserializer<'de, D>(de: D) -> Result<NaiveDateTime, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(de)?;
    Ok(NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%dT%H:%M:%S%z").unwrap())
}

pub fn datetime_serializer<S>(datetime: &NaiveDateTime, se: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let s = datetime.format("%Y-%m-%dT%H:%M:%S%z").to_string();
    se.collect_str(s.as_str())
}