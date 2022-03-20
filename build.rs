use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let scss_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("scss");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let css_path = out_dir.join("main.css");

    println!("cargo:rerun-if-changed={}", scss_dir.to_str().unwrap());

    let options = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .load_path(&scss_dir);
    let css = grass::from_path(scss_dir.join("main.scss").to_str().unwrap(), &options)?;

    fs::write(css_path, css)?;

    Ok(())
}
