mod core;
mod tools;
use crate::core::core::Core;
use std::{env, io::Read};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args();

    let core = Core::new(args)?;

    if let Err(err) = core.process() {
        eprintln!("Ошибка: {}", err)
    }

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    Ok(())
}
