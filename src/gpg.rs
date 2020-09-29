use gpgme::{Context, Key, Protocol};
use std::{fs::File, io::Result};

fn gpg_context() -> Result<Context> {
    let context = Context::from_protocol(Protocol::OpenPgp)?;
    Ok(context)
}

pub fn decrypt_file(path: &str) -> Result<Vec<u8>> {
    let mut gpg = gpg_context()?;

    let mut input = File::open(path)?;
    let mut output = Vec::new();
    gpg.decrypt(&mut input, &mut output)?;

    Ok(output)
}

pub fn encrypt_file(path: &str, recipients: Vec<String>, input: Vec<u8>) -> Result<()> {
    let mut gpg = gpg_context()?;
    let keys = get_keys(&recipients)?;

    let output = File::create(path)?;
    gpg.encrypt(&keys, input, output)?;

    Ok(())
}

fn get_keys(recipients: &Vec<String>) -> Result<Vec<Key>> {
    let keys = gpg_context()?
        .find_keys(recipients)?
        .filter_map(|x| x.ok())
        .filter(|x| x.can_encrypt())
        .collect();
    Ok(keys)
}
