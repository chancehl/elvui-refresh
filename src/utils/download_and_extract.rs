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
    // Create a new reqwest client
    let client = reqwest::Client::new();

    // Stream the response body of the latest tag zip
    let stream = client
        .get(url)
        .header(USER_AGENT, "elvui-refresh")
        .send()
        .await?;

    // Create a temporary directory
    let temp_dir = tempdir()?;

    // Define (temp) file path
    let path = temp_dir.path().join("addon.zip");

    // Create the (temporary) zipfile on disk
    let mut temp_file = File::create(&path)?;

    // Stream to a cursor
    let mut content = Cursor::new(stream.bytes().await?);

    // Copy content to temp .zip file
    io::copy(&mut content, &mut temp_file)?;

    // Create a new archive object
    let mut archive = ZipArchive::new(File::open(&path).expect("could not open file"))?;

    // Extract file
    archive.extract(&out_dir)?;

    // Parse out the top level directory (this is necessary because github includes a folder in the directory with the tag name, but Blizzard just wants the files)
    let elvui_dir = fs::read_dir(&out_dir)?
        .into_iter()
        .nth(0)
        .expect("Could not locate top level ElvUI directory")?
        .path();

    // Copy
    copy_directory(&elvui_dir, &out_dir)?;

    // Remove the top level tag directory so we don't have duplicate assets
    fs::remove_dir_all(elvui_dir)?;

    // Clean up temp dir
    temp_dir.close()?;

    Ok(())
}
