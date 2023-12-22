use std::fs::File;
use std::io::BufReader;
use time_tz::PrimitiveDateTimeExt as _;

/// See <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>:
///
/// `ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")`
///     \\ encodes \, \N or \n encodes newline
///     \; encodes ;, \, encodes ,
fn decode_text(encoded: &str) -> String {
    encoded
        .replace(r#"\\"#, r#"\"#)
        .replace(r#"\;"#, r#";"#)
        .replace(r#"\,"#, r#","#)
        .replace(r#"\N"#, "\n")
        .replace(r#"\n"#, "\n")
}

/// See <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5>, this is an ISO.8601 format,
/// and the timezone is specified externally.
///
/// Returns an Rfc2822 date time, which contains timezone info.
fn decode_date_time(property: &ical::property::Property) -> String {
    let Some(ref value) = property.value else {
        return "".into();
    };
    let ics_format = time::format_description::well_known::Iso8601::DEFAULT;
    let date_time = time::PrimitiveDateTime::parse(value, &ics_format).unwrap();
    let mut tzid = "".to_string();
    if let Some(ref params) = property.params {
        for (key, value) in params {
            if key == "TZID" {
                let time_zone = value.first();
                if let Some(value) = time_zone {
                    tzid = value.to_string();
                }
            }
        }
    }
    if tzid.is_empty() {
        return "".to_string();
    }
    let tz = time_tz::timezones::get_by_name(&tzid).unwrap();
    let date_time = date_time.assume_timezone(tz).unwrap();
    let format = time::format_description::well_known::Rfc2822;
    date_time.format(&format).unwrap()
}

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

fn handle_date_time(name: &str, property: &ical::property::Property) {
    let input_date = decode_date_time(property);
    if let Ok(improved) = improve_date(&input_date) {
        println!("{name}: {improved} ({input_date})");
    } else {
        println!("{name}: {input_date}");
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let buf = BufReader::new(File::open(args.next().unwrap()).unwrap());
    let reader = ical::IcalParser::new(buf);
    for calendar in reader {
        let calendar = calendar?;

        for event in calendar.events {
            for property in event.properties {
                if property.name == "SUMMARY" {
                    if let Some(value) = property.value {
                        println!("Summary: {}", decode_text(&value));
                    }
                } else if property.name == "DESCRIPTION" {
                    if let Some(value) = property.value {
                        println!("Description: {}", decode_text(&value));
                    }
                } else if property.name == "LOCATION" {
                    if let Some(value) = property.value {
                        println!("Location: {}", decode_text(&value));
                    }
                } else if property.name == "ORGANIZER" {
                    if let Some(value) = property.value {
                        println!("Organizer: {}", decode_text(&value));
                    }
                } else if property.name == "DTSTART" {
                    handle_date_time("Dtstart", &property);
                } else if property.name == "DTEND" {
                    handle_date_time("Dtend", &property);
                }
            }
        }
    }
    Ok(())
}
