mod command;
mod config;
mod gpg;
mod remote;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opts {
    #[structopt(name = "init", about = "Creates a new file")]
    Init {
        #[structopt(help = "Name for the file")]
        name: String,
        #[structopt(long, help = "Recipients that can open the file")]
        recipients: Vec<String>,
        #[structopt(long, help = "Remote location to syncronise the file")]
        remote: Option<String>,
    },
    #[structopt(name = "open", about = "Opens and decrypts a file")]
    Open,
    #[structopt(name = "shut", about = "Shuts and encrypts a file")]
    Shut,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    match opts {
        Opts::Init {
            name,
            recipients,
            remote,
        } => command::init(name, recipients, remote),
        Opts::Open => command::open(),
        Opts::Shut => command::shut(),
    }?;

    Ok(())
}
