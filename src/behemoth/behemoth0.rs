use anyhow::Result;
use crate::util::*;

pub fn solve(password: &str) -> Result<String> {
    // for behemoth 0, the password to the binary can be found by looking for strcmp in an ltrace
    // upon submitting the real password, it will open a shell

    let session = ssh_session(super::HOST, super::PORT, "behemoth0", password)?;

    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth0@gibson:~$ ")?;

    let test_pass = "test";

    let test_cmd = format!("echo {test_pass} | ltrace /behemoth/behemoth0 2>&1");
    println!("running '{test_cmd}'");
    write_line(&mut channel, &test_cmd)?;

    let result = read_until(&mut channel, "behemoth0@gibson:~$ ")?;
    let result = result.split("\n").skip(1).find(|s| s.contains(test_pass)).unwrap();
    println!("{result}");

    let real_pass = result.split("\"").nth(3).unwrap(); // strcmp("my_pass", "real_pass")
    println!("real pass is '{real_pass}'");

    let real_cmd = "/behemoth/behemoth0";
    println!("running '{real_cmd}' to spawn suid shell");
    write_line(&mut channel, &real_cmd)?;
    
    read_until(&mut channel, "Password: ")?;
    write_line(&mut channel, &real_pass)?;
    read_until(&mut channel, "$ ")?;

    println!("retrieving /etc/behemoth_pass/behemoth1");
    write_line(&mut channel, "cat /etc/behemoth_pass/behemoth1")?;

    let result = read_until(&mut channel, "$ ")?;
    let result = result.split("\n").nth(1).unwrap().trim();
    println!("retrieved behemoth1 pass '{result}'\n");

    Ok(result.to_string())
}