use clap::Parser;
use colored::Colorize;
use models::args::Args;
use models::tags::Tags;
use spinners::{Spinner, Spinners};
use std::{error::Error, process};
use utils::download_and_extract::download_and_extract;
use utils::logger::{LogLevel, Logger};

mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse args
    let args = Args::parse();

    // Instantiate logger
    let logger = &Logger::new();

    let addons_folder = match args.addons_folder {
        Some(dir) => {
            logger.info(format!("If you don't want to provide the -f/--addons-folder flag every time you execute this command, you can set the {} environment variable", "$BLIZZARD_ADDONS_FOLDER".yellow()));
            dir
        }
        None => {
            logger.log(LogLevel::Error, format!("You must provide either a {} flag specifying your Blizzard addons folder or set the {} variable", "-f/--addons-folder".yellow(), "$BLIZZARD_ADDONS_FOLDER".yellow()));
            process::exit(1);
        }
    };

    // Tell user we're fetching version info
    logger.info("Fetching latest ElvUI version from Github".to_string());

    // Query for github repo tags
    let tags = octocrab::instance()
        .get_tags("tukui-org".to_owned(), "ElvUI".to_owned())
        .await?;

    let latest = &tags[0];

    // Log latest version
    logger.info(format!("Latest version: {}", &latest.name.green()));

    // Instantiate spinner
    let mut download_spinner = Spinner::new(
        Spinners::Line,
        "Downloading and extracting latest version".to_string(),
    );

    // Download and extract file
    download_and_extract(&latest.zipball_url, addons_folder).await?;

    // Inform user of succcess
    download_spinner.stop_with_newline();

    logger.info(format!(
        "{} Upgraded ElvUI to version {}",
        "Success!".green(),
        &latest.name.green()
    ));

    Ok(())
}
