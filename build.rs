use std::{env, error::Error, fs, path::{PathBuf, Path}};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=articles");
    println!("cargo:rerun-if-changed=scss");

    let project_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    compile_scss(&project_dir, &out_dir)?;

    Ok(())
}

fn compile_scss(project_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let scss_dir = project_dir.join("scss");

    let css_path = out_dir.join("main.css");

    println!("cargo:rerun-if-changed=scss");

    let options = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .load_path(&scss_dir);
    let css = grass::from_path(scss_dir.join("main.scss").to_str().unwrap(), &options)?;

    fs::write(css_path, css)?;

    Ok(())
}
