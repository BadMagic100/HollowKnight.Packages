use schemars::schema::RootSchema;
use std::{env, fs, path::Path};
use typify::{TypeSpace, TypeSpaceSettings};

const SCHEMA_PATH: &str = "src/hpackage.schema.json";

fn main() {
    cargo_emit::rerun_if_changed!(SCHEMA_PATH);
    let schema_content = fs::read_to_string(SCHEMA_PATH).unwrap();
    let schema = serde_json::from_str::<RootSchema>(&schema_content).unwrap();

    let mut settings = TypeSpaceSettings::default();
    settings.with_struct_builder(true);

    let mut typespace = TypeSpace::new(&settings);
    typespace
        .add_root_schema(schema)
        .expect("Schema should convert successfully to typespace");

    let generated = typespace.to_stream().to_string();

    let content = format!(
        "
        #[allow(clippy::redundant_closure_call)]
        #[allow(clippy::needless_lifetimes)]
        #[allow(clippy::match_single_binding)]
        #[allow(clippy::clone_on_copy)]
        pub mod types {{
            use serde::{{Deserialize, Serialize}};
            {generated}
        }}"
    );

    let formatted = prettyplease::unparse(&syn::parse_str::<syn::File>(&content).unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("types.rs");
    fs::write(out_file, formatted).expect("Failed to write generated types file.");
}
