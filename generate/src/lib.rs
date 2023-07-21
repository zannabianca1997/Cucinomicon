use serde::{Deserialize, Serialize};
use serde_email::Email;
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    front_matter: FrontMatter,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    title: String,
    subtitle: String,
    author: String,
    email: Email,
    site: Url,
}

impl Book {}
