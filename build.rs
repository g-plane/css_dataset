use serde::Deserialize;
use std::{collections::HashMap, env, error::Error, fs, iter};

fn main() -> Result<(), Box<dyn Error>> {
    generate_properties()?;
    generate_properties_shorthand()?;

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

fn generate_properties_shorthand() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=vendor/properties-shorthand.json");

    let data =
        serde_json::from_str::<HashMap<String, Vec<String>>>(&fs::read_to_string(format!(
            "{}/vendor/properties-shorthand.json",
            env::var("CARGO_MANIFEST_DIR")?
        ))?)?;

    fs::write(
        format!("{}/css_properties_shorthand.rs", env::var("OUT_DIR")?),
        iter::once(format!(
            "{{\nlet mut map = std::collections::HashMap::with_capacity({});\n",
            data.len()
        ))
        .chain(data.into_iter().flat_map(|(key, value)| {
            iter::once(format!(
                "map.insert(\"{key}\", {{\nlet mut vec = Vec::with_capacity({});\n",
                value.len()
            ))
            .chain(
                value
                    .into_iter()
                    .map(|property| format!("vec.push(\"{property}\");\n")),
            )
            .chain(iter::once("vec\n});".to_string()))
        }))
        .chain(iter::once("map\n}".to_string()))
        .collect::<String>(),
    )?;

    Ok(())
}
