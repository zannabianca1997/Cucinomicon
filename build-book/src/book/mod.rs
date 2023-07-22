use std::{fs::File, io::read_to_string, path::Path};

use anyhow::Context;
use chrono::{DateTime, Utc};
use markdown::{Constructs, ParseOptions};
use serde::{Deserialize, Serialize};
use serde_email::Email;
use url::Url;

use crate::parsers::{
    headed_md::HeadedMarkdown, markdown::Markdown, title_separated_list::TitleSeparatedList, FromMd,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub front_matter: FrontMatter,
    pub introduction: Introduction,
}
impl Book {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading book from {}", path.as_ref().display());
        Ok(Self {
            front_matter: FrontMatter::load(path.as_ref().join("front_matter.yml"))
                .context("While loading `front_matter.yml`")?,
            introduction: Introduction::load(path.as_ref().join("introduction"))
                .context("While loading `introduction`")?,
        })
    }
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        let fm = self.front_matter.modified()?;
        let int = self.introduction.modified()?;
        Some(fm.max(int))
    }
}
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
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
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
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        self.modified
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Introduction {
    pub zen: HeadedMarkdown<Metas, TitleSeparatedList<Markdown, Markdown>>,
    pub prologue: HeadedMarkdown<Metas, Markdown>,
    pub warnings: HeadedMarkdown<Metas, TitleSeparatedList<Markdown, Markdown>>,
    pub thanks: HeadedMarkdown<Metas, Markdown>,
}
impl Introduction {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading introduction from {}", path.as_ref().display());

        fn load<T: FromMd>(path: &Path, name: &str) -> anyhow::Result<HeadedMarkdown<Metas, T>> {
            let path = path.join(name);
            log::debug!("Loading {}", path.display());
            let f = File::open(&path).context("While opening file")?;
            let mut content: HeadedMarkdown<Metas, T> = FromMd::parse(
                markdown::to_mdast(
                    &read_to_string(&f).context("While reading file")?,
                    &ParseOptions {
                        constructs: Constructs {
                            frontmatter: true, // needed to load the yaml too
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )
                .expect("Normal markdown should always parse"),
            )
            .context("While parsing")?;
            // adding modified date
            content.metas.modified = f
                .metadata()
                .and_then(|m| m.modified())
                .inspect_err(|err| {
                    log::warn!("Cannot read the modified time of {}: {err}", path.display())
                })
                .ok()
                .map(Into::into);

            anyhow::Ok(content)
        }

        Ok(Self {
            zen: load(path.as_ref(), "zen.md").context("While loading zen")?,
            prologue: load(path.as_ref(), "prologue.md").context("While loading prologue")?,
            warnings: load(path.as_ref(), "warnings.md").context("While loading warnings")?,
            thanks: load(path.as_ref(), "thanks.md").context("While loading thanks")?,
        })
    }
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        let mut modified = DateTime::<Utc>::MIN_UTC;
        for time in [
            &self.zen.metas.modified,
            &self.prologue.metas.modified,
            &self.warnings.metas.modified,
            &self.thanks.metas.modified,
        ] {
            // fail if a time is missing
            let time = *time.as_ref()?;
            // keep highest time
            modified = modified.max(time);
        }
        Some(modified)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metas {
    pub title: Markdown,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
}
