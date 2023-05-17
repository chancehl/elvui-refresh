use std::path::PathBuf;

use clap::Parser;

/// A program to download, extract and install the latest version of ElvUI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The addons folder location
    #[arg(short = 'f', long = "addons-folder")]
    pub addons_folder: Option<PathBuf>,
}
