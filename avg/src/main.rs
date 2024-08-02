/*
 * Copyright 2022 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Simply prints the average of all arguments.

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let count = (std::env::args().count() - 1) as i64;
    let mut iter = std::env::args();
    iter.next().context("next failed")?;
    let numbers: Result<Vec<_>, _> = iter
        .map(|arg| {
            // Strip away decimal and thousands separator from bash's time builtin.
            let stripped = arg.replace(['.', ','], "");
            stripped.parse::<i64>()
        })
        .collect();
    let sum: i64 = numbers?.iter().sum();
    println!("{}", sum.checked_div(count).context("checked_div failed")?);
    Ok(())
}
