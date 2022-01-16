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

    core.process()?;

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    Ok(())
}

/*
79112563636:dfdfgdfg
380669867856:gfhgfhfgh
0669863632:fghfghfhg
9112563698:gfhgfhfhg
89112563698:gfhfghdfg
7776984563:gghjghjg
+7 (911) 256-63-63:hgjghjgjh
38 066 987 45 66:gfhgfhfgh
8 (987) 455 77 99:gfhfghfhg
kvs88@nur.kz:ghjghjgjhg
+380 (66) 879 44 55:gfhfghfg
*/
