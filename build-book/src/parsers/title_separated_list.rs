//! Markdown file containing a list of heading separated paragraph

use anyhow::Context;
use markdown::mdast::{Heading, Node, Root};
use serde::{Deserialize, Serialize};

use super::{DisplayMd, FromMd};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TitleSeparatedList<Title, Content> {
    pub items: Vec<Item<Title, Content>>,
}
impl<T, C> FromMd for TitleSeparatedList<T, C>
where
    T: FromMd,
    C: FromMd,
{
    fn parse(mut md: Node) -> anyhow::Result<Self> {
        let mut items = vec![];
        for node in md.children_mut().unwrap().drain(..) {
            match node {
                Node::Heading(Heading {
                    children,
                    depth: 1,
                    position,
                }) => {
                    let title = T::parse(Node::Root(Root { children, position }))
                        .context("While parsing title")?;
                    items.push(Item {
                        title,
                        content: Node::Root(Root {
                            children: vec![],
                            position: None,
                        }),
                    })
                }
                _ => items
                    .last_mut()
                    .expect("File should begin with a heading")
                    .content
                    .children_mut()
                    .unwrap()
                    .push(node),
            }
        }
        Ok(Self {
            items: items
                .into_iter()
                .map(|Item { title, content }| {
                    anyhow::Ok(Item {
                        title,
                        content: C::parse(content).context("While parsing content")?,
                    })
                })
                .try_collect()?,
        })
    }
}

impl<T, C> DisplayMd for TitleSeparatedList<T, C>
where
    T: DisplayMd,
    C: DisplayMd,
{
    fn fmt(&self) -> anyhow::Result<Node> {
        Ok(Node::Root(Root {
            children: self
                .items
                .iter()
                .flat_map(|Item { title, content }| {
                    [
                        title.fmt().context("While displaying title").map(|md| {
                            Node::Heading(Heading {
                                position: md.position().cloned(),
                                children: vec![md],
                                depth: 1,
                            })
                        }),
                        content.fmt().context("While displaying content"),
                    ]
                })
                .try_collect()?,
            position: None,
        }))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item<Title, Content> {
    pub title: Title,
    pub content: Content,
}
