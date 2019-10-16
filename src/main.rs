use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opts {
    #[structopt(name = "init", about = "Creates a new file")]
    Init {
        name: String,
        remote: Option<String>,
    },
    #[structopt(name = "open", about = "Opens and decrypts a file")]
    Open,
    #[structopt(name = "shut", about = "Shuts and encrypts a file")]
    Shut,
}

fn main() {
    let opts = Opts::from_args();

    match opts {
        Opts::Init { name, remote } => file::init(name, remote),
        Opts::Open => file::open(),
        Opts::Shut => file::shut(),
    }
}
