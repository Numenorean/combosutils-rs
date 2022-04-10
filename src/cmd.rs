use std::{env, path::PathBuf};

use clap::{arg, Command, Error};

use crate::core::task::Task;

pub struct Args {
    pub task: Task,
    pub n: Option<usize>,
    pub targets: Vec<PathBuf>,
    pub binary_path: PathBuf,
}

pub fn parse_args() -> Result<Args, Error> {
    let matches = Command::new("combosutils-rs")
        .arg(arg!(--task <task>).possible_values(Task::possible_values()))
        .arg(
            arg!(-n <n> "Number of lines/parts")
                .required(false)
                .validator(|s| {
                    match s.parse::<usize>() {
                        Ok(n) => {
                            if n == 0 {
                                return Err(String::from("must be a number > 0"));
                            }
                        }
                        Err(_) => return Err(String::from("must be a number > 0")),
                    };
                    Ok(())
                }),
        )
        .arg(
            arg!(--target <target> "File(s) for processing")
                .required(true)
                .takes_value(true)
                .multiple_values(true),
        )
        .get_matches();
    let task: Task = matches.value_of_t("task")?;
    let n = matches.value_of("n").map(|s| s.parse::<usize>().unwrap());
    let targets: Vec<PathBuf> = matches
        .values_of("target")
        .unwrap()
        .map(PathBuf::from)
        .collect();

    let binary_path: PathBuf = env::args().next().unwrap().into();
    Ok(Args {
        task,
        n,
        targets,
        binary_path,
    })
}
