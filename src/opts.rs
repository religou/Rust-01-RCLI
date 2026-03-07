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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input CSV file", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, help = "Output Json file", default_value = "output.json")]
    pub output: String,

    #[arg(short, long, help = "Delimiter for CSV file", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "Include header row", default_value_t = true)]
    pub header: bool,
}

pub fn verify_file_exists(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File '{}' does not exist", filename))
    }
}
