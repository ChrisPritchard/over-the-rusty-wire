use std::io::Read;

use ssh::*;
use anyhow::Result;

const HOST: &str = "behemoth.labs.overthewire.org";
const PORT: usize = 2221;

fn main() -> Result<()> {

    let _pass_1 = behemoth0()?;

    Ok(())
}

fn ssh_session(username: &str, password: &str) -> Result<Session> {
    println!("connecting to server with username '{username}' and password '{password}'");

    let mut session = Session::new().unwrap();
    session.set_host(HOST)?;
    session.set_port(PORT)?;
    session.set_username(username)?;

    session.connect()?;
    match session.is_server_known()? {
        ServerKnown::Known => (),
        _ => session.write_knownhost()?
    }
    session.userauth_password(password)?;

    println!("connected successfully");

    Ok(session)
}

fn run_command<'b>(s: &mut Channel<'b>, cmd: &str) -> Result<(String, String)> {
    s.request_exec(cmd.as_bytes())?;
    s.send_eof().unwrap();

    let mut buf = Vec::new();
    s.stdout().read_to_end(&mut buf)?;
    let stdout_res = std::str::from_utf8(&mut buf)?;

    let mut buf = Vec::new();
    s.stderr().read_to_end(&mut buf)?;
    let stderr_res = std::str::from_utf8(&mut buf)?;

    Ok((stdout_res.to_string(), stderr_res.to_string()))
}

fn behemoth0() -> Result<String> {
    // for behemoth 0, the password to the binary can be found by looking for strcmp in an ltrace
    // upon submitting the real password, it will open a shell

    let mut session = ssh_session("behemoth0", "behemoth0")?;
    let mut s = session.channel_new()?;
    s.open_session()?;

    let test_pass = "test";
    let test_cmd = format!("echo {test_pass} | ltrace /behemoth/behemoth0");
    println!("running '{test_cmd}'");

    let (_, err) = run_command(&mut s, &test_cmd)?;
    let result = err.split("\n").find(|s| s.contains(test_pass)).unwrap();
    println!("{result}");

    let real_pass = result.split("\"").nth(3).unwrap(); // strcmp("my_pass", "real_pass")
    println!("real pass is '{real_pass}'");

    let real_cmd = format!("echo {real_pass} | /behemoth/behemoth0");
    println!("running '{real_cmd}'");

    let (out, _) = run_command(&mut s, &real_cmd)?;
    println!("{out}");

    println!("retrieving /etc/behemoth_pass/behemoth1");
    let (out, _) = run_command(&mut s, "cat /etc/behemoth_pass/behemoth1")?;

    println!("retrieved behemoth1 pass '{out}'");

    Ok(out)
}
