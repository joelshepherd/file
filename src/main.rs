use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opts {
    #[structopt(name = "init")]
    Init {
        name: String,
        remote: Option<String>,
    },
    #[structopt(name = "open")]
    Open,
    #[structopt(name = "shut")]
    Shut,
}

fn main() {
    let opts = Opts::from_args();

    match opts {
        Opts::Init { name, remote } => init(name, remote),
        Opts::Open => open(),
        Opts::Shut => shut(),
    }
}

fn init(name: String, remote: Option<String>) {
    println!("Initialising...");

    fs::create_dir(&name).unwrap();

    let path = format!("{}/.config", &name);
    let contents = format!(
        "NAME={}\nREMOTE={}",
        &name,
        &remote.unwrap_or(String::from(""))
    );
    fs::write(path, contents).unwrap();
}

fn open() {
    println!("Opening...");
}

fn shut() {
    println!("Shutting...");
}
