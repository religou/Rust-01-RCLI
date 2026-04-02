use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    prelude::*,
};
use std::{
    fs::File,
    io::{stdin, Read},
};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::STANDARD => STANDARD.encode(&buf),
        Base64Format::URLSAFE => URL_SAFE.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let decoded = match format {
        Base64Format::STANDARD => STANDARD.decode(&buf)?,
        Base64Format::URLSAFE => URL_SAFE.decode(&buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
