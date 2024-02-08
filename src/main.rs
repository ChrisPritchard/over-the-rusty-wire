use anyhow::Result;

mod util;
mod behemoth;

fn main() -> Result<()> {
    behemoth::solve_all()?;

    Ok(())
}
