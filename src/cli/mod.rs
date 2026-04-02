mod base64;
mod csv;
mod genpass;

// rcli csv -i input.csv -o output.json -d ',' --header
// rcli genpass -l 16 --uppercase --lowercase --numbers --symbols
pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or Process CSV files to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode or Decode base64 strings")]
    Base64(Base64SubCommand),
}

pub fn verify_file_exists(filename: &str) -> Result<String, String> {
    // if input is "-", or file is exists, return Ok
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File '{}' does not exist", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        // Test with existing file
        assert_eq!(verify_file_exists("Cargo.toml"), Ok("Cargo.toml".into()));

        // Test with non-existing file
        assert_eq!(
            verify_file_exists("not-existing.txt"),
            Err("File 'not-existing.txt' does not exist".into())
        );

        // Test with "-"
        assert_eq!(verify_file_exists("-"), Ok("-".into()));
    }
}
