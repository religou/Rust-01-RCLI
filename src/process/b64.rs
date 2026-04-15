use std::io::Read;

use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    prelude::*,
};

use crate::{utils::get_reader, Base64Format};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::STANDARD => STANDARD.encode(&buf),
        Base64Format::URLSAFE => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let decoded = match format {
        Base64Format::STANDARD => STANDARD.decode(&buf)?,
        Base64Format::URLSAFE => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
