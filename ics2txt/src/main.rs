/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! An ICS printer for mutt with detailed time info.

use anyhow::Context as _;
use ical::parser::Component as _;
use time_tz::PrimitiveDateTimeExt as _;

/// See <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>:
///
/// `ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")`
///     \\ encodes \, \N or \n encodes newline
///     \; encodes ;, \, encodes ,
fn decode_text(encoded: &str) -> String {
    encoded
        .replace(r"\\", r"\")
        .replace(r"\;", r";")
        .replace(r"\,", r",")
        .replace(r"\N", "\n")
        .replace(r"\n", "\n")
}

/// See <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5>, this is an ISO.8601 format,
/// and the timezone is specified externally.
///
/// Returns an Rfc2822 date time, which contains timezone info.
fn decode_date_time(property: &ical::property::Property) -> anyhow::Result<String> {
    let value = property.value.as_ref().context("no value")?;
    let ics_format = time::format_description::well_known::Iso8601::DEFAULT;
    let date_time = time::PrimitiveDateTime::parse(value, &ics_format)?;
    let params = property.params.as_ref().context("no params")?;
    let params_map: std::collections::HashMap<_, _> = params.iter().cloned().collect();
    let time_zone = params_map["TZID"].first().context("no TZID")?;
    let tz = time_tz::timezones::get_by_name(time_zone).context("can't find timezone")?;
    let time_tz::OffsetResult::Some(date_time) = date_time.assume_timezone(tz) else {
        return Err(anyhow::anyhow!("assume_timezone() failed"));
    };
    let format = time::format_description::well_known::Rfc2822;
    Ok(date_time.format(&format)?)
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

fn handle_date_time_property(
    name: &str,
    property: Option<&ical::property::Property>,
) -> anyhow::Result<()> {
    let Some(property) = property else {
        return Ok(());
    };

    let input_date = decode_date_time(property)?;
    if let Ok(improved) = improve_date(&input_date) {
        println!("{name}: {improved} ({input_date})");
    } else {
        println!("{name}: {input_date}");
    }

    Ok(())
}

fn handle_string_property(name: &str, property: Option<&ical::property::Property>) {
    let Some(property) = property else {
        return;
    };

    if let Some(ref value) = property.value {
        println!("{name}: {}", decode_text(value));
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let path = args.next().context("missing argument")?;
    let buf = std::io::BufReader::new(std::fs::File::open(path)?);
    let reader = ical::IcalParser::new(buf);
    for calendar in reader {
        let calendar = calendar?;

        for event in calendar.events {
            handle_string_property("Summary    ", event.get_property("SUMMARY"));
            handle_string_property("Description", event.get_property("DESCRIPTION"));
            handle_string_property("Location   ", event.get_property("LOCATION"));
            handle_string_property("Organizer  ", event.get_property("ORGANIZER"));
            handle_date_time_property("Dtstart    ", event.get_property("DTSTART"))?;
            handle_date_time_property("Dtend      ", event.get_property("DTEND"))?;
        }
    }
    Ok(())
}
