use std::io::BufRead as _;

/// Try to improve input_date by wrapping a non-local date in a local one.
fn improve_date(input_date: &str) -> anyhow::Result<String> {
    let format = time::format_description::well_known::Rfc2822;
    let mut date_time = time::OffsetDateTime::parse(input_date, &format)?;
    let local_offset = time::UtcOffset::current_local_offset()?;
    if date_time.offset() == local_offset {
        return Err(anyhow::anyhow!("matching offset"));
    }
    date_time = date_time.to_offset(local_offset);
    Ok(date_time.format(&format)?)
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut in_header = true;
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            in_header = false;
        }
        if in_header {
            if let Some(input_date) = line.strip_prefix("Date: ") {
                if let Ok(improved) = improve_date(input_date) {
                    println!("Date: {} ({})", improved, line);
                    continue;
                }
            }
        }
        println!("{}", line);
    }

    Ok(())
}
