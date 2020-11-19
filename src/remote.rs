use ssh2::Session;
use std::fs::File;
use std::io::{copy, Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use url::Url;

/// Pull a file from its remote
pub fn pull_file(remote: &str) -> Result<()> {
    let mut input: Box<dyn Read> = match parse_remote(remote) {
        Type::Remote(username, host, path) => {
            let session = start_session(&username, host)?;
            let (file, _) = session.scp_recv(&path)?;
            Box::new(file)
        }
        Type::Local(path) => Box::new(File::open(path)?),
    };
    let mut output = File::create(".file")?;

    copy(&mut input, &mut output)?;

    Ok(())
}

/// Push a file to a remote
pub fn push_file(remote: &str) -> Result<()> {
    let mut input = File::open(".file")?;
    let mut output: Box<dyn Write> = match parse_remote(remote) {
        Type::Remote(username, host, path) => {
            let session = start_session(&username, host)?;
            let file = session.scp_send(&path, 0o644, input.metadata()?.len(), None)?;
            Box::new(file)
        }
        Type::Local(path) => Box::new(File::create(path).unwrap()),
    };

    copy(&mut input, &mut output)?;

    Ok(())
}

enum Type {
    Remote(String, Vec<SocketAddr>, PathBuf),
    Local(PathBuf),
}

/// Parse remote into a struct
fn parse_remote(remote: &str) -> Type {
    match Url::parse(remote) {
        Ok(parsed) => Type::Remote(
            parsed.username().to_owned(),
            parsed
                .socket_addrs(|| None)
                .expect("Unable to resolve host"),
            Path::new(parsed.path()).to_owned(),
        ),
        Err(_) => Type::Local(Path::new(remote).to_owned()),
    }
}

/// Start a new ssh session
fn start_session(username: &str, host: Vec<SocketAddr>) -> Result<Session> {
    let stream = TcpStream::connect(&*host)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(stream);
    session.handshake()?;
    session.userauth_agent(username)?;

    Ok(session)
}
