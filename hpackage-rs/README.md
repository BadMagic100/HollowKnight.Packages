Rust client package for Hollow Knight PackageDefs. Types are generated from the schema at https://github.com/BadMagic100/HollowKnight.Packages using [typify](https://crates.io/crates/typify); see the docs for more details on the generated types.

Consumers may use the `parse_validate` function to validate and parse an input, but the generated types all play nicely with
`serde_json` if you'd rather skip validation or do it yourself.
