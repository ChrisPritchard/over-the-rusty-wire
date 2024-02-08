use anyhow::Result;

mod behemoth;
mod util;

fn main() -> Result<()> {
    behemoth::solve_all()?;

    Ok(())
}
