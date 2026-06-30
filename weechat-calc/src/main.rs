/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Simple calculator for weechat.

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (_, tokens) = args.split_first().context("args.split_first() failed")?;
    let expr = tokens.join(" ");
    let res = meval::eval_str(&expr).context("eval_str() failed")?;
    println!("{expr}={res}");
    Ok(())
}
