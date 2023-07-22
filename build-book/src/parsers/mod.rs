//! Some general purpose parser

use ::markdown::mdast::Node;

pub mod headed_md;
pub mod humantime_duration;
pub mod markdown;
pub mod string_or_struct;
pub mod title_separated_list;

pub trait FromMd: Sized {
    fn parse(md: Node) -> anyhow::Result<Self>;
}

pub trait DisplayMd {
    fn fmt(&self) -> anyhow::Result<Node>;
}
