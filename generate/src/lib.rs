use serde::{Deserialize, Serialize};
use serde_email::Email;

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
}

impl Book {}
