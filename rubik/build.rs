use kewb::{error::Error, fs::write_table};

fn main() -> Result<(), Error> {
    if !std::path::Path::new("bin/table.bin").exists() {
        std::fs::create_dir_all("bin")?;
        write_table("bin/table.bin")?;
    }

    Ok(())
}
