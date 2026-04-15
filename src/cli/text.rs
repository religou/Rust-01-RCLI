use std::{fmt, str::FromStr};

use super::verify_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a privated/shared key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a message with a signature")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, help = "Sign format", value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, help = "Input string to verify", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "Key to verify the text", value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, help = "Signature to verify")]
    pub sig: String,
    #[arg(long, help = "Sign format", value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    BLAKE3,
    ED25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl From<TextSignFormat> for &str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::BLAKE3 => "blake3",
            TextSignFormat::ED25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::BLAKE3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::anyhow!("Unsupported text sign format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
