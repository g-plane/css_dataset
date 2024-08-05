use serde::Deserialize;
use std::{env, error::Error, fs, iter};

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
        format!("{}/css_properties.rs", env::var("OUT_DIR")?),
        iter::once("[".to_string())
            .chain(
                data.properties
                    .into_iter()
                    .map(|property| format!("\"{property}\",")),
            )
            .chain(iter::once("]".to_string()))
            .collect::<String>(),
    )?;

    Ok(())
}
