use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    println!("Processing CSV file: {}", input);
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value: Value = headers.iter().zip(record.iter()).collect();
        ret.push(json_value);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
