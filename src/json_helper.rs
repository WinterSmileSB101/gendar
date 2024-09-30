use std::{fs::File, io::{Read, Result}};

use serde_json::Value;

pub fn parse_json<P: AsRef<std::path::Path>>(path: P) -> Result<Value> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let data: Value = serde_json::from_str(&contents)?;

    Ok(data)
}
