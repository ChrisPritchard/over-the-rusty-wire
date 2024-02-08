use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::Result;
use ssh2::*;

pub fn ssh_session(host: &str, port: u16, username: &str, password: &str) -> Result<Session> {
    println!("connecting to server with username '{username}' and password '{password}'");

    let tcp = TcpStream::connect(format!("{host}:{port}"))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password(username, password)?;
    println!("connected successfully");
    Ok(session)
}

pub fn create_shell(channel: &mut Channel) -> Result<()> {
    channel.request_pty("xterm", None, Some((80, 24, 0, 0)))?;
    channel.shell()?;
    Ok(())
}

pub fn read_until(channel: &mut Channel, finished_token: &str) -> Result<String> {
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

pub fn write_line(channel: &mut Channel, line: &str) -> Result<()> {
    channel.write(format!("{line}\n").as_bytes())?;
    channel.flush()?;
    Ok(())
}

pub fn hex_literal(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for b in bytes {
        encoded += &format!("\\x{:02x?}", b);
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::hex_literal;

    #[test]
    fn hex_literal_can_convert() {
        let bytes: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0x12, 0x34, 0x56];
        let encoded = hex_literal(&bytes);
        assert_eq!(encoded, "\\xde\\xad\\xbe\\xef\\x12\\x34\\x56");
    }
}
