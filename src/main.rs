#![allow(unused)]

use clap::Parser;
use std::path::PathBuf;
use chrono::prelude::{DateTime, Local, Utc};
use dirs;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    dir: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut dir = args.dir;
    let stash_root = match std::env::var("DESKSTASH_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(e) => dirs::home_dir().unwrap()
    };
    let stash_today = stash_root.join(Local::now().format("%Y-%m-%d").to_string());

    println!("{:?}", stash_today.into_os_string().into_string());

    // TODO: confirm today's stashdir/YYYY-MM-DD existence
    // if exist, make stashdir/YYYY-MM-DD.1

    // TODO: move ~/Desktop/* to stashdir
}
