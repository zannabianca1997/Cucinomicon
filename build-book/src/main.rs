use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use simple_logger::SimpleLogger;

use build_book::{frontends::Frontend, Book};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the book directory
    input: PathBuf,
    /// What frontend to use
    #[command(subcommand)]
    frontend: Frontend,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .env()
        .init()
        .context("Cannot init logger")?;
    let Args { frontend, input } = Parser::parse();
    log::info!("Reading book");
    let book = Book::load(input)?;
    log::info!("Writing output");
    frontend.emit(book)?;

    Ok(())
}
