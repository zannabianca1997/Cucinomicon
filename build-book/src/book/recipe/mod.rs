use std::{
    collections::BTreeMap,
    fs::{read_dir, File},
    io::read_to_string,
    mem,
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context};
use chrono::{DateTime, Duration, Utc};
use lazy_regex::regex_captures;
use markdown::{
    mdast::{Heading, List, Node, Root, Text},
    Constructs, ParseOptions,
};
use serde::{Deserialize, Serialize};

use crate::parsers::{headed_md::HeadedMarkdown, markdown::Markdown, FromMd};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recipe {
    pub name: Markdown,
    #[serde(with = "crate::parsers::humantime_duration")]
    pub time: Duration,
    pub ingredients: Vec<Ingredient>,
    pub tools: Vec<Markdown>,
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    pub descr: Markdown,
    pub preparazione: Vec<Markdown>,
    pub modifiche_e_aggiunte: Vec<Markdown>,
}
impl Recipe {
    pub fn load_dir(path: impl AsRef<Path>) -> anyhow::Result<BTreeMap<String, Self>> {
        log::info!("Loading recipes from {}", path.as_ref().display());

        let mut recipes = BTreeMap::new();

        for f in read_dir(path).context("Cannot read dir")? {
            let f = f.context("Cannot read directory entry")?;
            if !f.file_type().is_ok_and(|f| f.is_file()) {
                continue; // ignore all subdirectories
            }
            let Some(name) = f.file_name().to_string_lossy().into_owned().strip_suffix(".md").map(ToOwned::to_owned) else {continue; /* ignore all files not ending in .md */};
            let recipe = Self::load(f.path())
                .map_err(|err| err.context(format!("Error in parsing recipe {name}")))?;
            recipes.insert(name, recipe);
        }

        Ok(recipes)
    }
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading recipe from {}", path.as_ref().display());

        let f = File::open(path.as_ref()).context("Cannot open file")?;

        let HeadedMarkdown::<HumanHeader, HumanContent> { metas, content } = FromMd::parse(
            markdown::to_mdast(
                &read_to_string(&f).context("Cannot read file")?,
                &ParseOptions {
                    constructs: Constructs {
                        frontmatter: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .expect("Simple markdown should not panic"),
        )
        .context("While parsing")?;

        let modified = f
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

        Ok(Self {
            name: metas.name,
            time: metas.time,
            ingredients: metas.ingredients.into_iter().map(Into::into).collect(),
            tools: metas.tools,
            tags: metas.tags,
            modified,
            descr: content.descr,
            preparazione: content.preparazione,
            modifiche_e_aggiunte: content.modifiche_e_aggiunte,
        })
    }
    #[must_use]
    pub fn modified(&self) -> Option<DateTime<Utc>> {
        self.modified
    }
}

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ingredient {
    name: Markdown,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    comment: Option<Markdown>,
    #[serde(default, skip_serializing_if = "Quantity::is_to_taste")]
    quantity: Quantity,
    #[serde(default, skip_serializing_if = "is_false")]
    optional: bool,
}
impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_,name, comment, opt,q1,q2,unit)) = regex_captures!(r"^\s*(.+?)\s*(?:\(\s*(.+?)\s*\))?\s*(\?)?\s*(?:(\d+(?:\.\d+)?)(?:\s*\-\s*(\d+(?:\.\d+)?))?\s*((?:[[:punct:]]|[[:alpha:]]).*?))?\s*$", s) else {
            // bail!("Cannot recognize ingredient format: {}",s);
            unreachable!("The regex should always match")
        };
        if name.is_empty() {
            bail!("Need ingredient name!")
        }
        let name = Markdown::parse(markdown::to_mdast(name, &Default::default()).unwrap()).unwrap();
        let comment = if !comment.is_empty() {
            Some(
                Markdown::parse(markdown::to_mdast(comment, &Default::default()).unwrap()).unwrap(),
            )
        } else {
            None
        };
        let optional = !opt.is_empty();
        let quantity = match (q1, q2, unit) {
            ("", "", "") => Quantity::default(),
            (a, "", unit) => Quantity::Exact {
                n: a.parse()?,
                unit: match unit {
                    "" => None,
                    unit => Some(unit.to_owned()),
                },
            },
            (a, b, unit) => Quantity::Range {
                range: (a.parse()?, b.parse()?),
                unit: match unit {
                    "" => None,
                    unit => Some(unit.to_owned()),
                },
            },
        };
        Ok(Self {
            name,
            comment,
            quantity,
            optional,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum Quantity {
    #[default]
    ToTaste,
    Exact {
        n: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Range {
        range: (f64, f64),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
}

impl Quantity {
    /// Returns `true` if the quantity is [`ToTaste`].
    ///
    /// [`ToTaste`]: Quantity::ToTaste
    #[must_use]
    pub fn is_to_taste(&self) -> bool {
        matches!(self, Self::ToTaste)
    }
}

#[derive(Debug, Deserialize)]
struct HumanHeader {
    name: Markdown,
    #[serde(deserialize_with = "crate::parsers::humantime_duration::deserialize")]
    time: Duration,
    ingredients: Vec<HumanIngredient>,
    tools: Vec<Markdown>,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct HumanIngredient(
    #[serde(deserialize_with = "crate::parsers::string_or_struct::string_or_struct")] Ingredient,
);
impl From<HumanIngredient> for Ingredient {
    fn from(value: HumanIngredient) -> Self {
        value.0
    }
}

#[derive(Debug)]
struct HumanContent {
    descr: Markdown,
    preparazione: Vec<Markdown>,
    modifiche_e_aggiunte: Vec<Markdown>,
}
impl FromMd for HumanContent {
    fn parse(mut md: Node) -> anyhow::Result<Self> {
        let nodes = mem::take(
            md.children_mut()
                .context("Content need to split the different paragraphs")?,
        );

        let mut descr = vec![];
        let mut preparazione = vec![];
        let mut modifiche_e_aggiunte = vec![];

        let mut collecting = Some(&mut descr);

        for node in nodes {
            match node {
                Node::Heading(Heading {
                    children, depth: 1, ..
                }) => match &children[..] {
                    [Node::Text(Text { value, .. })] if value.trim() == "Preparazione" => {
                        collecting = Some(&mut preparazione)
                    }
                    [Node::Text(Text { value, .. })] if value.trim() == "Modifiche e aggiunte" => {
                        collecting = Some(&mut modifiche_e_aggiunte)
                    }
                    _ => {
                        log::warn!(
                            "Unrecognized header: {}\nThe content will be ignored",
                            Node::Heading(Heading {
                                children,
                                depth: 1,
                                position: None
                            })
                            .to_string()
                        );
                        collecting = None;
                    }
                },
                _ => {
                    if let Some(collecting) = &mut collecting {
                        collecting.push(node)
                    }
                }
            }
        }

        let descr: Markdown = FromMd::parse(Node::Root(Root {
            children: descr,
            position: None,
        }))
        .unwrap();

        let [Node::List(List{ children:steps, ordered: true, start:Some(1),.. })] = &mut preparazione[..] else {
            bail!("The `Preparazione` paragraph should be only a ordered list of steps, starting from 1")
        };
        let preparazione = mem::take(steps)
            .into_iter()
            .map(FromMd::parse)
            .try_collect()
            .unwrap();

        let [Node::List(List{ children:steps, ordered: false,.. })] = &mut modifiche_e_aggiunte[..] else {
            bail!("The `Modifiche e aggiunte` paragraph should be only a unordered list")
        };
        let modifiche_e_aggiunte = mem::take(steps)
            .into_iter()
            .map(FromMd::parse)
            .try_collect()
            .unwrap();

        Ok(Self {
            descr,
            preparazione,
            modifiche_e_aggiunte,
        })
    }
}
