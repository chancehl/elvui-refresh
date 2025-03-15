use super::logger::Logger;
use hyper::header::USER_AGENT;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Cursor},
    path::{Path, PathBuf},
};
use tempfile::tempdir;
use zip::ZipArchive;

pub async fn download_and_extract(url: &str, out_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let logger = Logger::new();
    let client = reqwest::Client::new();

    logger.info("Downloading addon source code...".to_string());

    let response = client
        .get(url)
        .header(USER_AGENT, "elvui-refresh")
        .send()
        .await?;

    let temp_dir = tempdir()?;
    logger.info(format!("Created temp directory at {:?}", temp_dir.path()));

    let temp_zip_path = temp_dir.path().join("addon.zip");
    let mut temp_file = File::create(&temp_zip_path)?;
    let mut content = Cursor::new(response.bytes().await?);
    io::copy(&mut content, &mut temp_file)?;

    logger.info("Extracting addon files...".to_string());

    let mut archive = ZipArchive::new(File::open(&temp_zip_path)?)?;
    archive.extract(temp_dir.path())?;

    logger.info("Copying addon files...".to_string());
    fs::remove_file(&temp_zip_path)?;

    // Allowed directories to copy
    let allowed_dirs = ["ElvUI", "ElvUI_Libraries", "ElvUI_Options"];

    // Identify the top-level extracted directory
    let extracted_root = fs::read_dir(temp_dir.path())?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().is_dir()) // Get the first directory
        .map(|entry| entry.path())
        .expect("Failed to find top-level directory in extracted zip");

    logger.info(format!(
        "Detected top-level directory: {:?}",
        extracted_root
    ));

    // Look for the allowed directories inside the top-level directory
    for entry in fs::read_dir(&extracted_root)? {
        let entry = entry?;
        let entry_name = entry.file_name();
        let entry_path = entry.path();

        if allowed_dirs.contains(&entry_name.to_string_lossy().as_ref()) {
            let dest_path = out_dir.join(&entry_name);
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&entry_path, &dest_path, &logger)?;
        }
    }

    logger.info("Cleaning up...".to_string());
    temp_dir.close()?;

    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path, logger: &Logger) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&src_path, &dest_path, logger)?;
        } else {
            if fs::metadata(&dest_path).is_ok() {
                logger.info(format!("Replacing file: {:?}", &dest_path));
            } else {
                logger.info(format!("Creating file: {:?}", &dest_path));
            }
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
