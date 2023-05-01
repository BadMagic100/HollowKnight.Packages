mod schema_types;

use jsonschema::JSONSchema;
use schema_types::HollowKnightPackageDef;

#[derive(Debug)]
pub enum ProcessingError {
    FailedToParse(serde_json::Error),
    FailedToValidate,
    FailedToDeserialize(serde_json::Error),
}

pub fn parse_validate(content: &str) -> Result<HollowKnightPackageDef, ProcessingError> {
    let schema_content = include_str!("../../hpackage.schema.json");
    let schema = serde_json::from_str(schema_content).expect("embedded schema");
    let compiled = JSONSchema::compile(&schema).expect("embedded schema");
    let instance = serde_json::from_str(content).map_err(ProcessingError::FailedToParse)?;

    let valid = compiled.is_valid(&instance);
    if !valid {
        return Err(ProcessingError::FailedToValidate);
    }

    serde_json::from_value(instance).map_err(ProcessingError::FailedToDeserialize)
}
