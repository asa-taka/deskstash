#![allow(unused)]

use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Local, Utc};
use clap::{AppSettings, Parser, Subcommand};
use dirs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Open stash directory
    Open {},
    /// Stash desktop files
    Stash {},
}

fn get_stash_root() -> PathBuf {
    match std::env::var("DESKSTASH_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(e) => dirs::home_dir()
            .expect("Home dir cannot detected.")
            .join(".deskstash"),
    }
}

fn get_stash_dir() -> PathBuf {
    get_stash_root().join(Local::now().format("%Y-%m-%d-%H%M%S").to_string())
}

fn open() -> io::Result<()> {
    std::process::Command::new("open")
        .arg(get_stash_root())
        .spawn()
        .expect("failed to execute process");
    Ok(())
}

fn stash() -> io::Result<()> {
    let dest_dir = get_stash_dir();
    let desktop = dirs::desktop_dir().expect("Desktop dir cannot detected.");

    fs::create_dir_all(&dest_dir)?;
    for e in fs::read_dir(desktop)? {
        let e = e?;
        let src = e.path();
        let dest = dest_dir.join(e.file_name());
        println!("{:?} → {:?}", src, dest);
        fs::rename(src, dest)?
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.command.unwrap_or(Command::Stash {}) {
        Command::Open {} => open(),
        Command::Stash {} => stash(),
    }
}
