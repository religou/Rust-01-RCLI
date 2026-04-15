use std::{fmt, str::FromStr};

use super::verify_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, help = "Input string to encode", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "encode way", value_parser = parse_base64_format, default_value = "STANDARD")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, help = "Base64 string to decode", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "decode way", value_parser = parse_base64_format, default_value = "STANDARD")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    STANDARD,
    URLSAFE,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::STANDARD => "STANDARD",
            Base64Format::URLSAFE => "URLSAFE",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::STANDARD),
            "url_safe" => Ok(Base64Format::URLSAFE),
            _ => Err(anyhow::anyhow!("Unsupported base64 format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
