mod core;
mod tools;
use std::{env, io::Read};

use mimalloc::MiMalloc;

use crate::core::core::Core;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args();

    let core = Core::new(args)?;

    core.process()?;

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    Ok(())
}
