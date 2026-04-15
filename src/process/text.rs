use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::*};
use core::str;
use std::{fs, io::Read};

trait TextSign {
    /// Sign the input data and return the signature as bytes
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    /// Verify the input data against the provided signature. Returns true if the signature is valid.
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let signed = match format {
        TextSignFormat::BLAKE3 => {
            let key = fs::read(key)?;
            let key = key.try_into().unwrap();
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::ED25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);
    Ok(())
}

pub fn process_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::BLAKE3 => {
            let key = fs::read(key)?;
            let key = key.try_into().unwrap();
            let verifier = Blake3 { key };
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::ED25519 => todo!(),
    };
    if verified {
        println!("Signature is valid");
    } else {
        println!("Signature is invalid");
    }
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}
