use crate::config::Config;
use crate::encrypt::{decrypt_file, encrypt_file};
use crate::remote::{pull_file, push_file};
use std::{fs, io::Result};
use tar::{Archive, Builder};

/// Initialised a new file
pub fn init(name: String, recipients: Vec<String>, remote: Option<String>) -> Result<()> {
    fs::create_dir(&name)?;

    let config = Config::new(recipients, remote);
    config.write(&name)?;

    Ok(())
}

/// Opens a shut file
pub fn open() -> Result<()> {
    let config = Config::read()?;
    if let Some(remote) = config.remote {
        pull_file(&remote)?;
    }

    let file = decrypt_file(".file")?;
    unpack_archive(file)?;
    fs::remove_file(".file")?;

    Ok(())
}

/// Shuts an opened file
pub fn shut() -> Result<()> {
    let config = Config::read()?;
    let files = find_files(".")?;

    let mut input = Vec::new();
    create_archive(&mut input, &files)?;
    encrypt_file(".file", config.recipients, input)?;

    for file in &files {
        fs::remove_file(file)?;
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
    let dir = fs::read_dir(path)?;
    for entry in dir {
        let path = entry?.path();
        let path = path.to_str().unwrap().to_owned();
        if path != "./.config" && path != "./.file.gpg" {
            files.push(path);
        }
    }
    Ok(files)
}
