use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opts {
    #[structopt(name = "init", about = "Creates a new file")]
    Init {
        #[structopt(help = "Name for the file")]
        name: String,
        #[structopt(help = "Recipients that can open the file")]
        recipients: Vec<String>,
    },
    #[structopt(name = "open", about = "Opens and decrypts a file")]
    Open,
    #[structopt(name = "shut", about = "Shuts and encrypts a file")]
    Shut,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    match opts {
        Opts::Init { name, recipients } => file::init(name, recipients),
        Opts::Open => file::open(),
        Opts::Shut => file::shut(),
    }?;

    Ok(())
}
