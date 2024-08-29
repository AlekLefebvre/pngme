use clap::Parser;
use std::path::PathBuf;

use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    
    #[command(subcommand)]
    pub(crate) command: Commands
}