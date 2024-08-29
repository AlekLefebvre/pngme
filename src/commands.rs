use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Encode chunk in png
    Encode {
        file: PathBuf,

        chunk_type: String,
        
        /// String to encode into png chunk
        content: String
    },

    /// Decode chunk in png
    Decode {
        file: PathBuf,

        chunk_type: String
    },

    /// Remove chunk from png
    Remove {
        file: PathBuf,

        chunk_type: String
    },

    /// Print png
    Print {
        file: PathBuf
    },
}