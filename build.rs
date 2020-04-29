use anyhow::{anyhow, Error};
use sass_rs::{compile_file, Options, OutputStyle};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    let mut opts = Options::default();
    opts.output_style = OutputStyle::Compact;
    let data = compile_file("./styles/yewtify.sass", opts)
        .map_err(|err| anyhow!("sass builder failed with: {}", err))?;
    let mut path = PathBuf::from(env::var("YEWTIFY_OUT")?);
    path.push("yewtify.css");
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
