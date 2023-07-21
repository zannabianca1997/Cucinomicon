use std::{io::stdout, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use generate::Book;
use simple_logger::SimpleLogger;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input directory
    #[clap(short, long)]
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .env()
        .init()
        .context("Cannot init logger")?;
    let Args { input } = Parser::parse();
    log::info!("Reading book");
    let book = Book::load(input)?;

    serde_yaml::to_writer(stdout(), &book).context("While serializing book")?;
    Ok(())
}
