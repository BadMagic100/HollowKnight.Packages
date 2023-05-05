use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!(schema = "src/hpackage.schema.json", struct_builder = true);

#[derive(Debug)]
pub enum ProcessingError {
    /// Indicates a failure to parse the raw json string. The serde error is provided.
    FailedToParse(serde_json::Error),
    /// Indicates that the schema failed validation. I haven't figured out how lifetimes work yet
    /// so the specific validations are not returned at this time.
    FailedToValidate,
    /// Indicates a failure to assign the validated raw JSON value into a PackageDef object.
    /// The serde error is provided.
    FailedToDeserialize(serde_json::Error),
}

/// Parses and validates a text string as a PackageDef
///
/// # Arguments
///
/// * `content` - the string of text content to parse.
///
/// # Examples
///
/// ```
/// use pretty_assertions::assert_eq;
/// use std::collections::HashMap;
/// use hpackage::{
///     HollowKnightPackageDef,
///     Asset,
///     References,
///     ReferenceVersion,
///     StringVersion,
///     parse_validate
/// };
///
/// let result = parse_validate(r#"
///     {
///         "name": "TheRealJournalRando",
///         "description": "Randomizer 4 addon that adds the option to randomize all other Hunter's Journal entries.",
///         "authors": ["BadMagic"],
///         "repository": "https://github.com/BadMagic100/TheRealJournalRando",
///         "dependencies": {
///             "ItemChanger": "@modlinks"
///         },
///         "devDependencies": {
///             "Randomizer 4": "@modlinks",
///             "ItemSync": "@modlinks",
///             "RandoSettingsManager": "@latest",
///             "MoreLocations": "@latest"
///         },
///         "assets": [
///             "bin/Publish/TheRealJournalRando.zip"
///         ]
///     }
/// "#).unwrap();
/// let builder = HollowKnightPackageDef::builder()
///     .name("TheRealJournalRando")
///     .description("Randomizer 4 addon that adds the option to randomize all other Hunter's Journal entries.")
///     .authors(vec!["BadMagic".to_string()])
///     .repository("https://github.com/BadMagic100/TheRealJournalRando".to_string())
///     .dependencies(References::from(HashMap::from([
///         ("ItemChanger".to_string(), ReferenceVersion::from(StringVersion("@modlinks".to_string())))
///     ])))
///     .dev_dependencies(References::from(HashMap::from([
///         ("Randomizer 4".to_string(), ReferenceVersion::from(StringVersion("@modlinks".to_string()))),
///         ("ItemSync".to_string(), ReferenceVersion::from(StringVersion("@modlinks".to_string()))),
///         ("RandoSettingsManager".to_string(), ReferenceVersion::from(StringVersion("@latest".to_string()))),
///         ("MoreLocations".to_string(), ReferenceVersion::from(StringVersion("@latest".to_string())))
///     ])))
///     .assets(Vec::from([
///         Asset::UniversalAsset("bin/Publish/TheRealJournalRando.zip".to_string())
///     ]));
/// let expected = HollowKnightPackageDef::try_from(builder).unwrap();
/// assert_eq!(
///     serde_json::to_value(result).unwrap(),
///     serde_json::to_value(expected).unwrap()
/// );
/// ```
///
/// # Failures
///
pub fn parse_validate(content: &str) -> Result<HollowKnightPackageDef, ProcessingError> {
    let schema_content = include_str!("hpackage.schema.json");
    let schema = serde_json::from_str(schema_content).expect("embedded schema");
    let compiled = JSONSchema::compile(&schema).expect("embedded schema");
    let instance = serde_json::from_str(content).map_err(ProcessingError::FailedToParse)?;

    let valid = compiled.is_valid(&instance);
    if !valid {
        return Err(ProcessingError::FailedToValidate);
    }

    serde_json::from_value(instance).map_err(ProcessingError::FailedToDeserialize)
}
