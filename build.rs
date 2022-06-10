use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use std::env;
use std::ffi::OsString;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    // let outdir = match env::var_os("OUT_DIR") {
        // None => return Ok(()),
        // Some(outdir) => outdir,
    // };

    let outdir: OsString = "target/completions/".into();
    let _ = std::fs::create_dir(&outdir);

    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    // let path_bash = generate_to(shells::Bash, &mut Opt::command(), BIN_NAME, &outdir)?;
    let path_zsh = generate_to(shells::Zsh, &mut Opt::command(), BIN_NAME, &outdir)?;

    // println!(
        // "cargo:warning=bash completion file is generated: {:?}",
        // path_bash
    // );
    println!(
        "cargo:warning=zsh completion file is generated: {:?}",
        path_zsh
    );

    Ok(())
}
