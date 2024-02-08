use anyhow::Result;

const HOST: &str = "utumno.labs.overthewire.org";
const PORT: u16 = 2227;

mod utumno0;

pub fn solve_latest() -> Result<()> {
    
    let _pass1 = utumno0::solve("utumno0")?;
    
    Ok(())
}