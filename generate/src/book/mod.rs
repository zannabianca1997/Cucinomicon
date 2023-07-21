use std::{borrow::Cow, fmt::Display, fs::File, path::Path, str::FromStr};

use anyhow::Context;
use markdown::mdast;
use serde::{Deserialize, Serialize};
use serde_email::Email;
use url::Url;
use zen::Zen;

#[derive(Debug, Clone)]
pub struct Markdown(mdast::Node);
impl Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
impl FromStr for Markdown {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = markdown::to_mdast(&s, &Default::default())
            .expect("Traditional markdown should never have errors");
        Ok(Self(node))
    }
}
impl Serialize for Markdown {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}
impl<'de> Deserialize<'de> for Markdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(<Cow<'de, str>>::deserialize(deserializer)?.parse().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    front_matter: FrontMatter,
    introduction: Introduction,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    title: Markdown,
    subtitle: Markdown,
    author: String,
    email: Email,
    site: Url,
}
impl FrontMatter {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading front matter from {}", path.as_ref().display());
        Ok(
            serde_yaml::from_reader(File::open(path).context("Cannot open `front_matter.yml`")?)
                .context("Cannot parse `front_matter.yml`")?,
        )
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Introduction {
    zen: Zen,
}
impl Introduction {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading introduction from {}", path.as_ref().display());
        Ok(Self {
            zen: Zen::load(path.as_ref().join("zen.md"))?,
        })
    }
}

mod zen;

impl Book {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading book from {}", path.as_ref().display());
        Ok(Self {
            front_matter: FrontMatter::load(path.as_ref().join("front_matter.yml"))?,
            introduction: Introduction::load(path.as_ref().join("introduction"))?,
        })
    }
}
