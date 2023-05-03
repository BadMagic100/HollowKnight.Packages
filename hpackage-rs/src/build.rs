use std::path::Path;
use std::process::Command;
use std::{env, str};

fn main() -> Result<(), failure::Error> {
    let git_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !git_output.status.success() {
        panic!("Couldn't find git repo root.")
    }
    let git_root = str::from_utf8(&git_output.stdout)?.trim();
    let source_file = format!(r"{git_root}/hpackage.schema.json");
    let out_dir = env::var("OUT_DIR").unwrap();

    println!(r"cargo:rerun-if-changed={source_file}");

    std::fs::copy(
        source_file,
        Path::new(&out_dir).join("hpackage.schema.json"),
    )?;

    Ok(())
}
