mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;

use std::path::PathBuf;
use std::str::FromStr;
use std::fs;

use crate::chunk_type::ChunkType;
use crate::commands::Commands;
use crate::cli::Cli;
use crate::png::Png;
use crate::chunk::Chunk;

fn load_file(file: &PathBuf) -> Png {
    let contents = fs::read(file.clone()).expect("Should have been able to read the file");
    let png = Png::try_from(contents.as_ref()).expect("PNG file isn't valid");
    return png;
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode { file, chunk_type, content } => {
            let mut png = load_file(file);
            let chunk_type = ChunkType::from_str(chunk_type.as_str());
            let chunk = Chunk::new(chunk_type.expect("Chunk type should be valid"), content.clone().into_bytes());
            let _ = png.append_chunk(chunk);
            fs::write(file.clone(), &png.as_bytes()).expect("Should have been able to write to the file");
        }
        Commands::Decode { file, chunk_type } => {
            let contents = fs::read(file.clone()).expect("Should have been able to read the file");
            let png = Png::try_from(contents.as_ref()).expect("PNG file isn't valid");
        
            println!("{}", &png.chunk_by_type(chunk_type.as_str()).expect("There are no chunk of that type"))
        }
        Commands::Remove { file, chunk_type } => {
            let contents = fs::read(file.clone()).expect("Should have been able to read the file");
            let mut png = Png::try_from(contents.as_ref()).expect("PNG file isn't valid");
        
            png.remove_first_chunk(chunk_type.as_str()).expect("Couldn't remove first chunk");
            fs::write(file, &png.as_bytes()).expect("Should have been able to write to the file");
        }
        Commands::Print { file } => {
            let contents = fs::read(file.clone()).expect("Should have been able to read the file");
            let png = Png::try_from(contents.as_ref()).expect("PNG file isn't valid");
        
            println!("{}", &png)
        }
    }

}