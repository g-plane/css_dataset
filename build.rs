use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("CARGO_FEATURE_CSS_PROPERTIES").is_ok() {
        generate_properties()?;
    }
    Ok(())
}

fn generate_properties() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=vendor/css-properties/data/all.json");
    let json = fs::read_to_string(format!(
        "{}/vendor/css-properties/data/all.json",
        env::var("CARGO_MANIFEST_DIR")?
    ))?;
    fs::write(
        format!("{}/css_properties.rs", env::var("OUT_DIR")?),
        json.trim_start_matches('{')
            .trim_start()
            .trim_start_matches("\"properties\":")
            .trim_end()
            .trim_end_matches('}'),
    )?;
    Ok(())
}
