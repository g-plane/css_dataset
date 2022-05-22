use serde::Deserialize;
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    generate_properties()?;

    Ok(())
}

fn generate_properties() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=vendor/css-properties/data/all.json");

    #[derive(Deserialize)]
    struct Data {
        properties: Vec<String>,
    }

    let data = serde_json::from_str::<Data>(&fs::read_to_string(format!(
        "{}/vendor/css-properties/data/all.json",
        env::var("CARGO_MANIFEST_DIR")?
    ))?)?;

    fs::write(
        format!("{}/css-properties.json", env::var("OUT_DIR")?),
        serde_json::to_string(&data.properties)?,
    )?;

    Ok(())
}
