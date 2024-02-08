use std::env;

use anyhow::Result;

mod behemoth;
mod utumno;
mod util;

fn main() -> Result<()> {
    
    if env::args().find(|a| a == "behemoth").is_some() {
        behemoth::solve_all()?;
    }
    
    utumno::solve_latest()?;

    Ok(())
}
