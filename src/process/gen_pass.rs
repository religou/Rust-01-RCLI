use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?/";

pub fn process_gen_pass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut charset: Vec<u8> = Vec::new();

    if uppercase {
        charset.extend(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if lowercase {
        charset.extend(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if numbers {
        charset.extend(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).unwrap());
    }
    if symbols {
        charset.extend(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).unwrap());
    }
    for _ in 0..(length - password.len() as u8) {
        let char = charset.choose(&mut rng).unwrap();
        password.push(*char);
    }
    password.shuffle(&mut rng);
    println!("{}", String::from_utf8(password)?);
    Ok(())
}
