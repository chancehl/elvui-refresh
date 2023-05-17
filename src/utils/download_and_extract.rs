use super::logger::Logger;
use fs_extra::{copy_items, dir::CopyOptions};
use hyper::header::USER_AGENT;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Cursor},
    path::PathBuf,
};
use tempfile::tempdir;
use zip::ZipArchive;

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
    let temp_zip_path = temp_dir.path().join("addon.zip");

    // Tell the user about the file we created
    logger.info(format!("Saving .zip to {:?}", &temp_zip_path));

    // Create the (temporary) zipfile on disk
    let mut temp_file = File::create(&temp_zip_path)?;

    // Stream to a cursor
    let mut content = Cursor::new(stream.bytes().await?);

    // Copy content to temp .zip file
    io::copy(&mut content, &mut temp_file)?;

    // Tell the user we're extracting their files
    logger.info(format!("Extracting addon files to {:?}", &temp_dir.path()));

    // Create a new archive object
    let mut archive = ZipArchive::new(File::open(&temp_zip_path).expect("could not open file"))?;

    // Extract file
    archive.extract(temp_dir.path())?;

    // Tell the we're copying their files
    logger.info(format!(
        "Copying addon files from {:?} to {:?}",
        &temp_dir.path(),
        &out_dir
    ));

    // Remove zip file after we're done so we don't copy it over
    fs::remove_file(temp_zip_path)?;

    // Parse out the top level directory (this is necessary because github includes a folder in the directory with the tag name, but Blizzard just wants the files)
    let nested_path = fs::read_dir(&temp_dir)?
        .map(|d| d.unwrap().path())
        .next()
        .expect("Failed to unwrap files");

    logger.info(format!(
        "Located nested ElvUI directories at {:?}",
        nested_path
    ));

    // Copy
    copy_items(
        &Vec::from([nested_path]),
        &out_dir,
        &CopyOptions::new().overwrite(true).copy_inside(true),
    )?;

    // Tell the user we're extracting their files
    logger.info(format!("Removing directory {:?}", &temp_dir.path()));

    // Clean up temp dir
    temp_dir.close()?;

    Ok(())
}
