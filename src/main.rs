use jsonschema::{is_valid, Draft, JSONSchema};
use reqwest;
use serde_json::{Value, json};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema: Value =
        reqwest::blocking::get("http://hl7.org/fhir/r4/fhir.schema.json")?.json()?;

    let compiled = JSONSchema::options()
        .compile(&schema)
        .expect("A valid schema");

    let contents =
        fs::read_to_string("Luci727_Murphy561_fdc21ebe-f012-0831-ccfc-28d243f4b5ac.json")
            .expect("Should have been able to read the file");
    let instance: Value = serde_json::from_str(&contents)?;
    match compiled.validate(&instance) {
        Ok(()) => {
            println!("Valid!!")
        }
        Err(errors) => {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
    }
    Ok(())
}
