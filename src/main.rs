use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opts {
    #[structopt(name = "init", about = "Creates a new file")]
    Init {
        #[structopt(help = "Name for the file")]
        name: String,
        #[structopt(help = "Who can open the file")]
        recipients: Vec<String>,
        #[structopt(long, help = "Remote to sync the file to")]
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
        Opts::Init {
            name,
            recipients,
            remote,
        } => file::init(name, recipients, remote),
        Opts::Open => file::open(),
        Opts::Shut => file::shut(),
    }
}
