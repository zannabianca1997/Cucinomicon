//! Markdown container that is (de)serialized from a string

use std::borrow::Cow;

use markdown::mdast::Node;
use serde::{Deserialize, Serialize};

use super::{DisplayMd, FromMd};

#[derive(Debug, Clone)]
pub struct Markdown(pub Node);
impl FromMd for Markdown {
    fn parse(md: Node) -> anyhow::Result<Self> {
        Ok(Self(md))
    }
}
impl DisplayMd for Markdown {
    fn fmt(&self) -> anyhow::Result<Node> {
        Ok(self.0.clone())
    }
}
impl Serialize for Markdown {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.fmt().unwrap().to_string().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Markdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <Cow<'de, str>>::deserialize(deserializer)?;
        let md = markdown::to_mdast(&s, &Default::default())
            .expect("Normal markdown has no format error");
        Ok(Self::parse(md).unwrap())
    }
}
