//! Markdown file headed with a yaml header

use anyhow::Context;
use markdown::mdast::{Node, Root, Yaml};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{DisplayMd, FromMd};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HeadedMarkdown<Metas, Content> {
    pub metas: Metas,
    pub content: Content,
}

impl<M, C> FromMd for HeadedMarkdown<M, C>
where
    M: DeserializeOwned,
    C: FromMd,
{
    fn parse(mut md: Node) -> anyhow::Result<Self> {
        let metas = md
            .children_mut()
            .unwrap()
            .drain_filter(|node| matches!(node, markdown::mdast::Node::Yaml(_)))
            .map(|node| {
                if let Node::Yaml(Yaml { value, .. }) = node {
                    serde_yaml::from_str::<M>(&value).context("Cannot parse metadata")
                } else {
                    unreachable!()
                }
            })
            .next()
            .context("Cannot find yaml metadata")
            .flatten()?;
        let content = C::parse(md).context("Cannot parse content")?;
        Ok(Self { metas, content })
    }
}

impl<'s, M, C: 's> DisplayMd for HeadedMarkdown<M, C>
where
    M: Serialize,
    C: DisplayMd,
{
    fn fmt(&self) -> anyhow::Result<Node> {
        let Self { metas, content } = self;
        let metas = Node::Yaml(Yaml {
            value: serde_yaml::to_string(metas).context("While formatting metadata")?,
            position: None,
        });
        let mut content = content.fmt().context("While formatting content")?;
        if let Some(childs) = content.children_mut() {
            childs.insert(0, metas);
        } else {
            content = Node::Root(Root {
                children: vec![metas, content],
                position: None,
            });
        }
        Ok(content)
    }
}
