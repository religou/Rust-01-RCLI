use rand::seq::IndexedRandom;

pub fn process_gen_pass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = String::new();
    let mut charset: Vec<u8> = Vec::new();

    if uppercase {
        charset.extend(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
    }
    if lowercase {
        charset.extend(b"abcdefghijkmnopqrstuvwxyz");
    }
    if numbers {
        charset.extend(b"123456789");
    }
    if symbols {
        charset.extend(b"!@#$%^&*()-_=+[]{}|;:,.<>?/");
    }
    for _ in 0..length {
        let char = charset.choose(&mut rng).unwrap();
        password.push(*char as char);
    }
    println!("Generated password: {}", password);
    Ok(())
}
