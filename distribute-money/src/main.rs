/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Commandline interface to distribute-money.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let csv_path = args.next().context("no first arg")?;
    let csv_file = std::fs::File::open(csv_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv_file);
    let mut accounts: Vec<distribute_money::Account> = Vec::new();
    for result in reader.deserialize() {
        let row: distribute_money::Account = result?;
        accounts.push(row);
    }

    let transactions = distribute_money::distribute_money(&accounts);
    println!("from\tto\tamount");
    for transaction in &transactions {
        println!(
            "{}\t{}\t{}",
            transaction.from, transaction.to, transaction.amount
        );
    }

    Ok(())
}
