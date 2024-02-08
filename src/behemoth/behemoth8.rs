use crate::util::*;
use anyhow::Result;

/// final level, no challenge here :)
pub fn solve(password: &str) -> Result<()> {
    let session = ssh_session(super::HOST, super::PORT, "behemoth8", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "$ ")?;

    println!("reading final message:");
    write_line(&mut channel, "cat CONGRATULATIONS")?;
    let result = read_until(&mut channel, "$ ")?;
    println!("{result}");

    Ok(())
}
