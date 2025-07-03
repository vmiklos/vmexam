/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! A typography unit converter.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Unit {
    Inch,
    Point,
    Twip,
    M,
    Cm,
    Mm,
    Mm100,
    Emu,
    Pixel,
}

impl TryFrom<&str> for Unit {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inch" | "inches" => Ok(Unit::Inch),
            "point" | "points" => Ok(Unit::Point),
            "twip" | "twips" => Ok(Unit::Twip),
            "m" | "ms" => Ok(Unit::M),
            "cm" | "cms" => Ok(Unit::Cm),
            "mm" | "mms" => Ok(Unit::Mm),
            "mm100" | "mm100s" => Ok(Unit::Mm100),
            "emu" | "emus" => Ok(Unit::Emu),
            "pixel" | "pixels" => Ok(Unit::Pixel),
            _ => Err(anyhow::anyhow!("invalid unit value: {value}")),
        }
    }
}

fn convert(amount: f64, fro: Unit, to: Unit) -> anyhow::Result<f64> {
    let conv: std::collections::HashMap<Unit, f64> = [
        (Unit::Inch, 914400.0),               // "there are 914,400 EMUs per inch"
        (Unit::Point, 914400.0 / 72.0),       // EMU / point
        (Unit::Twip, 914400.0 / 72.0 / 20.0), // EMU / twip
        (Unit::M, 360.0 * 100000.0),          // EMU / m
        (Unit::Cm, 360.0 * 1000.0),           // EMU is defined as 1/360,000 of a centimeter
        (Unit::Mm, 360.0 * 100.0),            // EMU / mm
        (Unit::Mm100, 360.0),                 // EMU / mm100
        (Unit::Emu, 1.0),                     // EMU / EMU
        (Unit::Pixel, 914400.0 / 96.0),       // CSS pixel, so 96 DPI
    ]
    .into_iter()
    .collect();

    // convert to EMU
    let emu = amount * conv.get(&fro).context("unexpected unit")?;
    Ok(emu / conv.get(&to).context("unexpected unit")?)
}

/// Inner main() that is allowed to fail.
pub fn our_main(argv: Vec<String>, stream: &mut dyn std::io::Write) -> anyhow::Result<()> {
    let amount_arg = clap::Arg::new("amount").index(1);
    let from_arg = clap::Arg::new("from").index(2);
    let in_arg = clap::Arg::new("in").index(3);
    let to_arg = clap::Arg::new("to").index(4);
    let args = [amount_arg, from_arg, in_arg, to_arg];
    let app = clap::Command::new("tpconv");
    let args = app.args(&args).try_get_matches_from(argv)?;
    let amount = args
        .get_one::<String>("amount")
        .context("amount is required")?;
    let amount: f64 = amount.parse()?;
    let from: &str = args.get_one::<String>("from").context("from is required")?;
    let from: Unit = from.try_into()?;
    let to: &str = args.get_one::<String>("to").context("to is required")?;
    let to: Unit = to.try_into()?;
    writeln!(stream, "{}", convert(amount, from, to)?)?;
    Ok(())
}

/// Similar to plain main(), but with an interface that allows testing.
pub fn main(args: Vec<String>, stream: &mut dyn std::io::Write) -> i32 {
    match our_main(args, stream) {
        Ok(_) => 0,
        Err(err) => {
            stream.write_all(format!("{err:?}\n").as_bytes()).unwrap();
            1
        }
    }
}

#[cfg(test)]
mod tests;
