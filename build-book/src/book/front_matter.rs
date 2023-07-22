use std::{fs::File, path::Path};

use anyhow::Context;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_email::Email;
use url::Url;

use crate::parsers::markdown::Markdown;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: Markdown,
    pub subtitle: Markdown,
    pub author: String,
    pub email: Email,
    pub site: Url,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
}
impl FrontMatter {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading front matter from {}", path.as_ref().display());
        let f = File::open(&path).context("While opening file")?;
        let mut frontmatters: FrontMatter =
            serde_yaml::from_reader(&f).context("Cannot parse file")?;
        frontmatters.modified = f
            .metadata()
            .and_then(|m| m.modified())
            .inspect_err(|err| {
                log::warn!(
                    "Cannot read the modified time of {}: {err}",
                    path.as_ref().display()
                )
            })
            .ok()
            .map(Into::into);
        Ok(frontmatters)
    }

    #[must_use]
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        self.modified
    }
}
