use std::fs::File;
use std::io::BufReader;

/// See <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>:
///
/// `ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")`
///     \\ encodes \, \N or \n encodes newline
///     \; encodes ;, \, encodes ,
fn decode_property_value(encoded: &str) -> String {
    encoded
        .replace(r#"\\"#, r#"\"#)
        .replace(r#"\;"#, r#";"#)
        .replace(r#"\,"#, r#","#)
        .replace(r#"\N"#, "\n")
        .replace(r#"\n"#, "\n")
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
                        println!("Summary: {}", decode_property_value(&value));
                    }
                } else if property.name == "DESCRIPTION" {
                    if let Some(value) = property.value {
                        println!("Description: {}", decode_property_value(&value));
                    }
                } else if property.name == "LOCATION" {
                    if let Some(value) = property.value {
                        println!("Location: {}", decode_property_value(&value));
                    }
                } else if property.name == "ORGANIZER" {
                    if let Some(value) = property.value {
                        println!("Organizer: {}", decode_property_value(&value));
                    }
                } else if property.name == "DTSTART" {
                    if let Some(value) = property.value {
                        println!("Dtstart: {}", decode_property_value(&value));
                    }
                    if let Some(params) = property.params {
                        for (key, value) in params {
                            if key == "TZID" {
                                let time_zone = value.first();
                                if let Some(value) = time_zone {
                                    println!("Dtstart Tzid: {}", value);
                                }
                            }
                        }
                    }
                } else if property.name == "DTEND" {
                    if let Some(value) = property.value {
                        println!("Dtend: {}", decode_property_value(&value));
                    }
                    if let Some(params) = property.params {
                        for (key, value) in params {
                            if key == "TZID" {
                                let time_zone = value.first();
                                if let Some(value) = time_zone {
                                    println!("Dtend Tzid: {}", value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
