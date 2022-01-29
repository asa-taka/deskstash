#![allow(unused)]

use std::fs;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Local, Utc};
use clap::Parser;
use dirs;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    dir: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let mut dir = args.dir;
    let stash_root = match std::env::var("DESKSTASH_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(e) => dirs::home_dir()
            .expect("Home dir cannot detected.")
            .join(".deskstash"),
    };
    let stash_today = stash_root.join(Local::now().format("%Y-%m-%d-%H%M%S").to_string());
    let desktop = dirs::desktop_dir().expect("Desktop dir cannot detected.");

    fs::create_dir_all(&stash_today)?;
    for e in fs::read_dir(desktop)? {
        let e = e?;
        println!("{:?}→{:?}", e.path(), stash_today.join(e.file_name()));
        fs::rename(e.path(), stash_today.join(e.file_name()))?
    }
    Ok(())
}
