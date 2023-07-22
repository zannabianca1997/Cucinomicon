use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Context;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

pub mod front_matter;
use self::front_matter::FrontMatter;

pub mod introduction;
use self::introduction::Introduction;

pub mod recipe;
use self::recipe::Recipe;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub front_matter: FrontMatter,
    pub introduction: Introduction,
    pub recipes: BTreeMap<String, Recipe>,
}
impl Book {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading book from {}", path.as_ref().display());
        Ok(Self {
            front_matter: FrontMatter::load(path.as_ref().join("front_matter.yml"))
                .context("While loading `front_matter.yml`")?,
            introduction: Introduction::load(path.as_ref().join("introduction"))
                .context("While loading `introduction`")?,
            recipes: Recipe::load_dir(path.as_ref().join("recipes"))
                .context("While loading `recipes`")?,
        })
    }

    #[must_use]
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        let mut max = DateTime::<Utc>::MIN_UTC;
        max = max.max(self.front_matter.modified()?);
        max = max.max(self.introduction.modified()?);
        for (_, r) in &self.recipes {
            max = max.max(r.modified()?)
        }
        Some(max)
    }
}
