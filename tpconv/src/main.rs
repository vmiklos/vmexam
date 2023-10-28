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
use clap::Parser as _;

#[derive(Clone, Eq, Hash, PartialEq, clap::ValueEnum)]
enum Unit {
    #[value(alias("inches"))]
    Inch,
    #[value(alias("points"))]
    Point,
    #[value(alias("twips"))]
    Twip,
    #[value(alias("ms"))]
    M,
    #[value(alias("cms"))]
    Cm,
    #[value(alias("mms"))]
    Mm,
    #[value(alias("mm100s"))]
    Mm100,
    #[value(alias("emus"))]
    Emu,
    #[value(alias("pixels"))]
    Pixel,
}

#[derive(clap::Parser)]
struct Arguments {
    amount: f64,
    fro: Unit,
    _in: String,
    to: Unit,
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

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    println!("{}", convert(args.amount, args.fro, args.to)?);
    Ok(())
}
