use std::{io::stdout, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use simple_logger::SimpleLogger;

use build_book::{frontends::Frontend, Book};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What frontend to use
    #[command(subcommand)]
    frontend: Frontend,
    /// Path to the book directory
    #[clap(short, long)]
    input: PathBuf,
    /// Path to the output
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .env()
        .init()
        .context("Cannot init logger")?;
    let Args {
        frontend,
        input,
        output,
    } = Parser::parse();
    log::info!("Reading book");
    let book = Book::load(input)?;
    log::info!("Writing output");
    frontend.emit(book, output)?;

    Ok(())
}
