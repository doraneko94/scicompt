use std::fs::File;
use std::io::{Write};

#[allow(bare_trait_objects)]
pub fn save_to_file(filename: &str, s: &str) -> Result<(), Box<std::error::Error>> {
    let mut file = File::create(filename)?;
    file.write_all(s.as_bytes())?;
    file.flush()?;
    Ok(())
}