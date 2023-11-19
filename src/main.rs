use jsonschema::{is_valid, Draft, JSONSchema};
use reqwest;
use serde_json::{json, Value};
use std::fs;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Opts {
    /// File path
    #[clap(long)]
    file_path: String,

    /// FHIR version
    #[clap(value_enum, long)]
    fhir_version: Vec<FhirVersion>,
}

#[derive(Clone, ValueEnum, Debug)]
enum FhirVersion {
    Stu3,
    R4,
    R5,
}

impl FhirVersion {
    fn get_schema_url(&self) -> &'static str {
        match self {
            FhirVersion::Stu3 => "http://hl7.org/fhir/stu3/fhir.schema.json",
            FhirVersion::R4 => "http://hl7.org/fhir/r4/fhir.schema.json",
            FhirVersion::R5 => "http://hl7.org/fhir/r5/fhir.schema.json",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    println!("File path: {}", opts.file_path);
        let contents =
            fs::read_to_string(opts.file_path).expect("Should have been able to read the file");
        let instance: Value = serde_json::from_str(&contents)?;

    for fhir_version in opts.fhir_version.iter() {
        println!("FHIR version: {:?}", fhir_version);

        let schema: Value = reqwest::blocking::get(fhir_version.get_schema_url())?.json()?;

        let compiled = JSONSchema::options()
            .compile(&schema)
            .expect("A valid schema");

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
        };
    }
    Ok(())
}
