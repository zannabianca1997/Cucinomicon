//! Some general purpose parser

use ::markdown::mdast::Node;

pub mod headed_md;
pub mod markdown;
pub mod title_separated_list;

pub trait FromMd: Sized {
    fn parse(md: Node) -> anyhow::Result<Self>;
}
pub trait DisplayMd {
    fn fmt(&self) -> anyhow::Result<Node>;
}