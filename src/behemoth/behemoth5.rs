use crate::util::*;
use anyhow::Result;

/// the code for behemoth5 is a bit messy in ghidra, but basically it opens the pass file,
/// creates a udp socket to localhost 1337, and sends the password through before closing.
/// to get it, we only need to listen there with nc.
pub fn solve(password: &str) -> Result<String> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth5", password)?;
    ssh.read_until("$ ")?;

    println!("creating a tmp folder and moving to it");
    ssh.write_line("cd $(mktemp -d)")?;
    ssh.read_until("$ ")?;

    println!("starting a nc UDP listener");
    ssh.write_line("nc -lu localhost 1337 > out.txt &")?;
    ssh.read_until("$ ")?;

    println!("running target");
    ssh.write_line("/behemoth/behemoth5")?;
    ssh.read_until("$ ")?;

    ssh.write_line("cat out.txt")?;
    let result = ssh.read_until("$ ")?;
    let result = result
        .split(['\r', '\n'])
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth5 pass '{result}'\n");

    Ok(result.to_string())
}
