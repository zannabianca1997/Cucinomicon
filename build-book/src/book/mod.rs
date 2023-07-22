use std::{
    fs::{read_to_string, File},
    path::Path,
};

use anyhow::Context;
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
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: Markdown,
    pub subtitle: Markdown,
    pub author: String,
    pub email: Email,
    pub site: Url,
}
impl FrontMatter {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading front matter from {}", path.as_ref().display());
        Ok(
            serde_yaml::from_reader(File::open(path).context("Cannot open file")?)
                .context("Cannot parse file")?,
        )
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

        fn load<T: FromMd>(path: &Path, name: &str) -> anyhow::Result<T> {
            let path = path.join(name);
            log::debug!("Loading {}", path.display());
            anyhow::Ok(
                FromMd::parse(
                    markdown::to_mdast(
                        &read_to_string(path).context("While reading")?,
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
                .expect("While parsing"),
            )
        }

        Ok(Self {
            zen: load(path.as_ref(), "zen.md").context("While loading zen")?,
            prologue: load(path.as_ref(), "prologue.md").context("While loading prologue")?,
            warnings: load(path.as_ref(), "warnings.md").context("While loading warnings")?,
            thanks: load(path.as_ref(), "thanks.md").context("While loading thanks")?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metas {
    pub title: Markdown,
}
