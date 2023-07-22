#![feature(const_trait_impl)]
#![feature(result_flattening)]
#![feature(drain_filter)]
#![feature(never_type)]
#![feature(iterator_try_collect)]
#![feature(is_some_and)]
#![feature(result_option_inspect)]

pub mod book;
pub use book::Book;

pub(crate) mod parsers;

pub mod frontends {
    use std::path::Path;

    use clap::Subcommand;

    use crate::Book;

    pub mod yaml {
        use std::{fs::File, path::Path};

        use anyhow::Context;

        use crate::Book;

        pub fn emit(book: Book, output: impl AsRef<Path>) -> anyhow::Result<()> {
            log::info!("Writing yaml book to {}", output.as_ref().display());
            serde_yaml::to_writer(File::create(output).context("Cannot create file")?, &book)
                .context("While writing file")?;
            Ok(())
        }
    }

    #[derive(Debug, Subcommand)]
    pub enum Frontend {
        YAML,
    }
    impl Frontend {
        pub fn emit(&self, book: Book, output: impl AsRef<Path>) -> anyhow::Result<()> {
            match self {
                Frontend::YAML => yaml::emit(book, output),
            }
        }
    }
}
