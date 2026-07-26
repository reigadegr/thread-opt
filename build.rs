#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
use anyhow::Result;
use vergen::{Build, Cargo, Emitter, Rustc, Sysinfo};

fn main() -> Result<()> {
    vergen()?;
    Ok(())
}

fn vergen() -> Result<()> {
    let build = Build::all_build();
    let cargo = Cargo::all_cargo();
    let rustc = Rustc::all_rustc();
    let si = Sysinfo::all_sysinfo();

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .emit()
}
