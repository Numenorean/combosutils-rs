mod core;
mod errors;
mod processors;
use crate::core::core::Core;
use std::{env, io::Read};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[allow(clippy::unused_io_amount)]
fn main() {
    let args = env::args();

    let core = match Core::new(args) {
        Ok(core) => core,
        Err(error) => {
            eprintln!("Ошибка при инициализации: {}", error);
            std::io::stdin().read(&mut [0u8]).unwrap();
            return;
        }
    };

    if let Err(err) = core.process() {
        eprintln!("Ошибка при обработке: {}", err)
    }

    std::io::stdin().read(&mut [0u8]).unwrap();
}
