use super::verify_file_exists;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input CSV file", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, help = "Output Json file")]
    pub output: Option<String>,

    #[arg(long, help = "Output format", value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, help = "Delimiter for CSV file", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "Include header row", default_value_t = true)]
    pub header: bool,
}

pub fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Unsupported output format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
