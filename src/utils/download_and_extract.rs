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
    logger.info("Downloading addon source code...".to_string());

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

    // Create the (temporary) zipfile on disk
    let mut temp_file = File::create(&temp_zip_path)?;

    // Stream to a cursor
    let mut content = Cursor::new(stream.bytes().await?);

    // Copy content to temp .zip file
    io::copy(&mut content, &mut temp_file)?;

    // Tell the user we're extracting their files
    logger.info("Extracting addon files...".to_string());

    // Create a new archive object
    let mut archive = ZipArchive::new(File::open(&temp_zip_path).expect("could not open file"))?;

    // Extract file
    archive.extract(temp_dir.path())?;

    // Tell the we're copying their files
    logger.info("Copying addon files...".to_string());

    // Remove zip file after we're done so we don't copy it over
    fs::remove_file(temp_zip_path)?;

    // Parse out the top level directory (this is necessary because github includes a folder in the directory with the tag name, but Blizzard just wants the files)
    let addon_files_path = fs::read_dir(&temp_dir)?
        .map(|d| d.unwrap().path())
        .next()
        .expect("Failed to unwrap files");

    let addon_files =
        fs::read_dir(&addon_files_path)?.map(|d| d.expect("Could not parse addon file").path());

    for addon_file in addon_files {
        // copy
        copy_items(
            &Vec::from([addon_file]),
            &out_dir,
            &CopyOptions::new().overwrite(true),
        )?;
    }

    // Tell the user we're extracting their files
    logger.info("Cleaning up...".to_string());

    // Clean up temp dir
    temp_dir.close()?;

    Ok(())
}
