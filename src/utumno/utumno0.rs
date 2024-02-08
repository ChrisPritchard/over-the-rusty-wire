use crate::util::*;
use anyhow::Result;

pub fn solve(password: &str) -> Result<()> {
    let session = ssh_session(super::HOST, super::PORT, "utumno0", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "$ ")?;

    Ok(())
}