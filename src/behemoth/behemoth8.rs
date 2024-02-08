use crate::util::*;
use anyhow::Result;

/// final level, no challenge here :)
pub fn solve(password: &str) -> Result<()> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth8", password)?;
    ssh.read_until("$ ")?;

    println!("reading final message:");
    ssh.write_line("cat CONGRATULATIONS")?;
    let result = ssh.read_until("$ ")?;
    println!("{result}");

    Ok(())
}
