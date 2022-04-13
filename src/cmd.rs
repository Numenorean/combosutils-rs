use std::{env, path::PathBuf};

use clap::{arg, Command, Error};

use crate::core::task::Task;

pub struct Args {
    pub task: Task,
    pub n: Option<usize>,
    pub targets: Vec<PathBuf>,
    pub compare_with: Option<PathBuf>,
    pub binary_path: PathBuf,
}

pub fn parse_args() -> Result<Args, Error> {
    let matches = Command::new("combosutils-rs")
        .arg(arg!(--task <task>).possible_values(Task::possible_values()))
        .arg(
            arg!(-n <n> "Number of lines/parts")
                .required(false)
                .takes_value(true)
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
        .arg(
            arg!(--with <file> "Compare with file/dir")
                .required(true)
                .takes_value(true)
                .validator(|s| {
                    let path: PathBuf = s.to_owned().into();
                    if !path.exists() {
                        return Err(String::from("path does not exist"));
                    }
                    println!("{:?}", path);
                    Ok(())
                }),
        )
        .get_matches();
    let task: Task = matches.value_of_t("task")?;
    let n = matches.value_of("n").map(|s| s.parse::<usize>().unwrap());
    let targets: Vec<PathBuf> = matches
        .values_of("target")
        .unwrap()
        .map(PathBuf::from)
        .collect();
    let compare_with = matches.value_of("with").map(PathBuf::from);

    let binary_path: PathBuf = env::args().next().unwrap().into();
    Ok(Args {
        task,
        n,
        targets,
        compare_with,
        binary_path,
    })
}
