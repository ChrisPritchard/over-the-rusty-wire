use crate::util::*;
use anyhow::Result;

/// for behemoth 0, the password to the binary can be found by looking for strcmp in an ltrace
/// upon submitting the real password, it will open a shell
pub fn solve(password: &str) -> Result<String> {

    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth0", password)?;
    ssh.read_until("$ ")?;

    let test_pass = "test";

    let test_cmd = format!("echo {test_pass} | ltrace /behemoth/behemoth0 2>&1");
    println!("running '{test_cmd}'");
    ssh.write_line(&test_cmd)?;

    let result = ssh.read_until("$ ")?;
    let result = result
        .split("\n")
        .skip(1)
        .find(|s| s.contains(test_pass))
        .unwrap();
    println!("{result}");

    let real_pass = result.split("\"").nth(3).unwrap(); // strcmp("my_pass", "real_pass")
    println!("real pass is '{real_pass}'");

    let real_cmd = "/behemoth/behemoth0";
    println!("running '{real_cmd}' to spawn suid shell");
    ssh.write_line(&real_cmd)?;

    ssh.read_until("Password: ")?;
    ssh.write_line(&real_pass)?;
    ssh.read_until("$ ")?;

    println!("retrieving /etc/behemoth_pass/behemoth1");
    ssh.write_line("cat /etc/behemoth_pass/behemoth1")?;

    let result = ssh.read_until("$ ")?;
    let result = result.split("\n").nth(1).unwrap().trim();
    println!("retrieved behemoth1 pass '{result}'\n");

    Ok(result.to_string())
}
