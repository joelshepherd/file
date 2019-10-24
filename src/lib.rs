use gpgme::{Context, Key, Protocol};
use serde::{Deserialize, Serialize};
use std::{fs, iter::Iterator, str};
use tar::{Archive, Builder};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    recipients: Vec<String>,
    remote: Option<String>,
}

pub fn init(name: String, recipients: Vec<String>, remote: Option<String>) {
    fs::create_dir(&name).unwrap();

    let path = format!("{}/.config", &name);
    let config = Config {
        name,
        recipients,
        remote,
    };
    let contents = toml::to_string_pretty(&config).unwrap();
    fs::write(path, contents).unwrap();
}

pub fn open() {
    let mut input = fs::File::open(".file.gpg").unwrap();
    let mut output = Vec::new();
    gpg_context().decrypt(&mut input, &mut output).unwrap();

    let mut archive = Archive::new(&output[..]);
    archive.unpack(".").unwrap();

    shred(".file.gpg");
}

pub fn shut() {
    let config = read_config(".");
    let files = find_files(".");
    let keys = convert_keys(&config.recipients);

    // Create and encrypt the archive
    let mut input = Vec::new();
    let mut output = fs::File::create(".file.gpg").unwrap();
    append_archive(&mut input, &files);
    gpg_context()
        .encrypt(&keys, &mut input, &mut output)
        .unwrap();

    // Shred files
    for file in &files {
        shred(file);
    }
}

fn append_archive(write: &mut Vec<u8>, files: &Vec<String>) {
    let mut archive = Builder::new(write);
    for file in files {
        archive.append_path(file).unwrap();
    }
    archive.finish().unwrap();
}

fn convert_keys(recipients: &Vec<String>) -> Vec<Key> {
    gpg_context()
        .find_keys(recipients)
        .unwrap()
        .filter_map(|x| x.ok())
        .filter(|k| k.can_encrypt())
        .collect()
}

fn find_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        let path = path.to_str().unwrap();
        if path != "./.config" && path != "./.file.gpg" {
            files.push(path.to_string());
        }
    }
    files
}

fn gpg_context() -> Context {
    Context::from_protocol(Protocol::OpenPgp).unwrap()
}

fn read_config(path: &str) -> Config {
    let path = format!("{}/.config", path);
    let contents = fs::read_to_string(path).unwrap();
    toml::from_str(&contents).unwrap()
}

fn shred(path: &str) {
    fs::remove_file(path).unwrap()
}
