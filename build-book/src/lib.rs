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
    use std::path::PathBuf;

    use clap::Subcommand;

    use crate::Book;

    #[cfg(feature = "frontend-yaml")]
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

    #[cfg(feature = "frontend-html")]
    pub mod html {
        use std::{
            fs::{create_dir_all, File},
            path::Path,
        };

        use anyhow::Context;

        use crate::Book;

        pub fn emit(book: Book, output: impl AsRef<Path>) -> anyhow::Result<()> {
            log::info!("Writing html book to {}", output.as_ref().display());
            create_dir_all(output.as_ref()).context("Cannot create output dir")?;
            Ok(())
        }
    }

    #[derive(Debug, Subcommand)]
    pub enum Frontend {
        /// Create a YAML representation of the book
        #[cfg(feature = "frontend-yaml")]
        Yaml {
            /// Path to the output
            output: PathBuf,
        },
        /// Create a static html representation of the book
        #[cfg(feature = "frontend-html")]
        Html {
            /// Path to the output directory
            output: PathBuf,
        },
        /// Only checks for errors
        Check,
    }
    impl Frontend {
        pub fn emit(&self, book: Book) -> anyhow::Result<()> {
            match self {
                #[cfg(feature = "frontend-yaml")]
                Frontend::Yaml { output } => yaml::emit(book, output),
                #[cfg(feature = "frontend-html")]
                Frontend::Html { output } => html::emit(book, output),
                Frontend::Check => {
                    log::info!("Book builded successfully!");
                    Ok(())
                }
            }
        }
    }
}
