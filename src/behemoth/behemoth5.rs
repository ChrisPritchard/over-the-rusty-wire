use anyhow::Result;
use crate::util::*;

pub fn solve(password: &str) -> Result<String> {
    // the code for behemoth5 is a bit messy in ghidra, but basically it opens the pass file,
    // creates a udp socket to localhost 1337, and sends the password through before closing.
    // to get it, we only need to listen there with nc.

    let session = ssh_session(super::HOST, super::PORT, "behemoth5", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;
    read_until(&mut channel, "$ ")?;

    println!("creating a tmp folder and moving to it");
    write_line(&mut channel, "cd $(mktemp -d)")?;
    read_until(&mut channel, "$ ")?;

    println!("starting a nc UDP listener");
    write_line(&mut channel, "nc -lu localhost 1337 > out.txt &")?;
    read_until(&mut channel, "$ ")?;

    println!("running target");
    write_line(&mut channel, "/behemoth/behemoth5")?;
    read_until(&mut channel, "$ ")?;

    write_line(&mut channel, "cat out.txt")?;
    let result = read_until(&mut channel, "$ ")?;
    let result = result.split(['\r','\n']).map(|s| s.trim()).find(|s| s.len() == 10).unwrap();
    println!("retrieved behemoth5 pass '{result}'\n");

    Ok(result.to_string())
}