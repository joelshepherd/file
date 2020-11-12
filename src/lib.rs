mod gpg;
mod remote;
use remote::{pull_file, push_file};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, read_dir, read_to_string, remove_file, write},
    io::Result,
};
use tar::{Archive, Builder};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    recipients: Vec<String>,
    remote: Option<String>,
}

/// Initialised a new file
pub fn init(name: String, recipients: Vec<String>, remote: Option<String>) -> Result<()> {
    create_dir(&name)?;
    write_config(&name, &Config { recipients, remote })?;

    Ok(())
}

/// Opens a shut file
pub fn open() -> Result<()> {
    let config = read_config()?;
    if let Some(remote) = config.remote {
        pull_file(&remote)?;
    }

    let file = gpg::decrypt_file(".file")?;
    unpack_archive(file)?;
    remove_file(".file")?;

    Ok(())
}

/// Shuts an opened file
pub fn shut() -> Result<()> {
    let config = read_config()?;
    let files = find_files(".")?;

    let mut input = Vec::new();
    create_archive(&mut input, &files)?;
    gpg::encrypt_file(".file", config.recipients, input)?;

    for file in &files {
        remove_file(file)?;
    }
    if let Some(remote) = config.remote {
        push_file(&remote)?;
    }

    Ok(())
}

fn create_archive(output: &mut Vec<u8>, files: &Vec<String>) -> Result<()> {
    let mut archive = Builder::new(output);
    for file in files {
        archive.append_path(file)?;
    }
    archive.finish()?;
    Ok(())
}

fn unpack_archive(input: Vec<u8>) -> Result<()> {
    let mut archive = Archive::new(&input[..]);
    archive.unpack(".")?;
    Ok(())
}

fn find_files(path: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();
    let dir = read_dir(path)?;
    for entry in dir {
        let path = entry?.path();
        let path = path.to_str().unwrap().to_owned();
        if path != "./.config" && path != "./.file.gpg" {
            files.push(path);
        }
    }
    Ok(files)
}

fn read_config() -> Result<Config> {
    let contents = read_to_string(".config")?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

fn write_config(name: &str, config: &Config) -> Result<()> {
    let path = format!("{}/.config", name);
    // @todo Pretty display formatted unwrap
    let input = toml::to_string_pretty(config).unwrap();
    write(path, input)?;
    Ok(())
}
