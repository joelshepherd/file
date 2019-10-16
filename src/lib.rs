use gpgme::{Context, Protocol};
use std::fs;
use tar::{Archive, Builder};

pub fn init(name: String, remote: Option<String>) {
    fs::create_dir(&name).unwrap();

    let path = format!("{}/.config", &name);
    let contents = format!(
        "NAME={}\nREMOTE={}",
        &name,
        &remote.unwrap_or(String::from(""))
    );
    fs::write(path, contents).unwrap();

    println!("Done");
}

pub fn open() {
    let mut input = fs::File::open(".file.gpg").unwrap();
    let mut output = Vec::new();
    gpg_context().decrypt(&mut input, &mut output).unwrap();

    let mut archive = Archive::new(&output[..]);
    archive.unpack(".").unwrap();

    shred(".file.gpg");

    println!("Done");
}

pub fn shut() {
    let mut input = Vec::new();
    {
        let mut archive = Builder::new(&mut input);
        archive.append_dir_all(".", ".").unwrap();
        archive.finish().unwrap();
    }

    let mut output = Vec::new();
    gpg_context()
        .encrypt_symmetric(&mut input, &mut output)
        .unwrap();

    fs::write(".file.gpg", &output).unwrap();

    for entry in fs::read_dir(".").unwrap() {
        let path = entry.unwrap().path();
        let path = path.to_str().unwrap();
        if path != "./.config" && path != "./.file.gpg" {
            shred(path);
        }
    }

    println!("Done");
}

fn gpg_context() -> Context {
    Context::from_protocol(Protocol::OpenPgp).unwrap()
}

fn shred(path: &str) {
    fs::remove_file(path).unwrap()
}
