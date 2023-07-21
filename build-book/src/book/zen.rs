use std::fs::read_to_string;
use std::path::Path;

use anyhow::Context;
use markdown::{
    mdast::{Heading, Node, Root, Yaml},
    Constructs, ParseOptions,
};
use serde::{Deserialize, Serialize};

use super::Markdown;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Zen {
    pub title: Markdown,
    pub rules: Vec<Rule>,
}
impl Zen {
    pub(crate) fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        log::info!("Loading zen from {}", path.as_ref().display());
        let md = read_to_string(path).context("Cannot read file")?;
        let mut md = markdown::to_mdast(
            &md,
            &ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .map_err(|err| anyhow::format_err!(err))
        .context("Cannot parse file")?;
        // searching for the frontmatter
        let title = md
            .children_mut()
            .unwrap()
            .drain_filter(|node| matches!(node, markdown::mdast::Node::Yaml(_)))
            .map(|node| {
                if let Node::Yaml(Yaml { value, .. }) = node {
                    serde_yaml::from_str::<Metas>(&value).context("Cannot parse metadata")
                } else {
                    unreachable!()
                }
            })
            .next()
            .context("Cannot find yaml metadata")
            .flatten()?
            .title
            .parse()
            .unwrap();
        // splitting the file into rules
        let mut rules = vec![];
        for node in md.children_mut().unwrap().drain(..) {
            match node {
                Node::Heading(Heading {
                    children,
                    depth: 1,
                    position,
                }) => {
                    let title = Markdown(Node::Root(Root { children, position }));
                    log::debug!("Reading rule {}", title);
                    rules.push(Rule {
                        title,
                        descr: Markdown(Node::Root(Root {
                            children: vec![],
                            position: None,
                        })),
                    })
                }
                _ => rules
                    .last_mut()
                    .expect("The zen should begin with the heading of the first rule")
                    .descr
                    .0
                    .children_mut()
                    .unwrap()
                    .push(node),
            }
        }
        Ok(Self { title, rules })
    }
}

#[derive(Deserialize)]
struct Metas {
    title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rule {
    pub title: Markdown,
    pub descr: Markdown,
}
