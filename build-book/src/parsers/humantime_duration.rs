use anyhow::Context;
use chrono::Duration;
use serde::{de::Error as DeError, ser::Error as SerError, Deserializer, Serializer};

pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let std = duration
        .to_std()
        .context("A negative duration was serialized")
        .map_err(S::Error::custom)?;
    humantime_serde::serialize(&std, serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let std = humantime_serde::deserialize(deserializer)?;
    Ok(Duration::from_std(std)
        .context("A duration too big was deserialized")
        .map_err(D::Error::custom)?)
}
