use std::io::Read;

use ssh::*;
use anyhow::Result;

const HOST: &str = "behemoth.labs.overthewire.org";
const PORT: usize = 2221;

fn main() -> Result<()> {
    let pass_1 = behemoth0()?;

    Ok(())
}

fn behemoth0() -> Result<String> {
    let mut session = Session::new().unwrap();
    session.set_host(HOST)?;
    session.set_port(PORT)?;
    session.set_username("behemoth0")?;

    session.connect()?;
    match session.is_server_known()? {
        ServerKnown::Known => (),
        _ => session.write_knownhost()?
    }
    session.userauth_password("behemoth0")?;

    {
        let mut s = session.channel_new()?;
        s.open_session()?;
        s.request_exec(b"ls -laR /behemoth")?;
        s.send_eof().unwrap();
        let mut buf = Vec::new();
        s.stdout().read_to_end(&mut buf)?;
        println!("{:?}", std::str::from_utf8(&buf)?)
    }

    Ok("".into())
}
