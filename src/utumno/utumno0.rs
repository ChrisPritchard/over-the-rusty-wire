use crate::util::*;
use anyhow::Result;

pub fn solve(password: &str) -> Result<()> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "utumno0", password)?;
    ssh.read_until("$ ")?;

    Ok(())
}