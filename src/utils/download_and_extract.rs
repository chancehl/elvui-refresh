use super::logger::Logger;
use hyper::header::USER_AGENT;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Cursor},
    path::PathBuf,
};
use tempfile::tempdir;
use zip::ZipArchive;

use super::copy_directory::copy_directory;

pub async fn download_and_extract(url: &str, out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    // Instantiate logger
    let logger = Logger::new();

    // Create a new reqwest client
    let client = reqwest::Client::new();

    // Tell user we're downloading
    logger.info("Downloading zip file from Github".to_string());

    // Stream the response body of the latest tag zip
    let stream = client
        .get(url)
        .header(USER_AGENT, "elvui-refresh")
        .send()
        .await?;

    // Create a temporary directory
    let temp_dir = tempdir()?;

    // Tell the user where we're (temporarily) saving things
    logger.info(format!("Created temp directory at {:?}", &temp_dir.path()));

    // Define (temp) file path
    let path = temp_dir.path().join("addon.zip");

    // Tell the user about the file we created
    logger.info(format!("Saving .zip to {:?}", &path));

    // Create the (temporary) zipfile on disk
    let mut temp_file = File::create(&path)?;

    // Stream to a cursor
    let mut content = Cursor::new(stream.bytes().await?);

    // Copy content to temp .zip file
    io::copy(&mut content, &mut temp_file)?;

    // Tell the user we're extracting their files
    logger.info(format!("Extracting addon files to {:?}", &temp_dir.path()));

    // Create a new archive object
    let mut archive = ZipArchive::new(File::open(&path).expect("could not open file"))?;

    // Extract file
    archive.extract(&temp_dir.path())?;

    // Tell the we're copying their files
    logger.info(format!(
        "Copying addon files from {:?} to {:?}",
        &temp_dir.path(),
        &out_dir
    ));

    // Parse out the top level directory (this is necessary because github includes a folder in the directory with the tag name, but Blizzard just wants the files)
    let elvui_dir = fs::read_dir(&temp_dir)?
        .next()
        .expect("Could not locate top level ElvUI directory")?
        .path();

    // Copy
    copy_directory(&elvui_dir, &out_dir)?;

    // Remove the top level tag directory so we don't have duplicate assets
    fs::remove_dir_all(elvui_dir)?;

    // Tell the user we're extracting their files
    logger.info(format!(
        "Removing files at {:?} (note: the directory will be removed as well)",
        &temp_dir.path()
    ));

    // Clean up temp dir
    temp_dir.close()?;

    Ok(())
}
