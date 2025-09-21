use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    // pub parttern: String,
    pub path: std::path::PathBuf,
    
}