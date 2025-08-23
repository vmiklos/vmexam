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

#[derive(Clone, Debug, serde::Deserialize)]
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: &str, balance: i64) -> Self {
        let owner = owner.to_string();
        Account { owner, balance }
    }
}

struct Transaction {
    from: String,
    to: String,
    amount: i64,
}

impl Transaction {
    fn new(from: &str, to: &str, amount: i64) -> Self {
        let from = from.to_string();
        let to = to.to_string();
        Transaction { from, to, amount }
    }
}

fn distribute_money(accounts: &[Account]) -> Vec<Transaction> {
    let mut creditors: Vec<Account> = accounts.iter().filter(|i| i.balance > 0).cloned().collect();
    // Order big amounts first, i.e. reverse sort.
    creditors.sort_by(|a, b| b.balance.cmp(&a.balance));
    let mut debtors: Vec<Account> = accounts
        .iter()
        .filter(|i| i.balance < 0)
        .map(|i| Account::new(&i.owner, -i.balance))
        .collect();
    debtors.sort_by(|a, b| b.balance.cmp(&a.balance));

    let mut transactions: Vec<Transaction> = Vec::new();

    let mut debtors_iter = debtors.iter_mut().peekable();
    let mut creditors_iter = creditors.iter_mut().peekable();

    while let (Some(debtor), Some(creditor)) = (debtors_iter.peek_mut(), creditors_iter.peek_mut())
    {
        let amount = std::cmp::min(debtor.balance, creditor.balance);
        transactions.push(Transaction::new(&debtor.owner, &creditor.owner, amount));

        debtor.balance -= amount;
        creditor.balance -= amount;

        if debtor.balance == 0 {
            debtors_iter.next();
        }
        if creditor.balance == 0 {
            creditors_iter.next();
        }
    }

    transactions
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let csv_path = args.next().context("no first arg")?;
    let csv_file = std::fs::File::open(csv_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv_file);
    let mut accounts: Vec<Account> = Vec::new();
    for result in reader.deserialize() {
        let row: Account = result?;
        accounts.push(row);
    }

    let transactions = distribute_money(&accounts);
    println!("from\tto\tamount");
    for transaction in &transactions {
        println!(
            "{}\t{}\t{}",
            transaction.from, transaction.to, transaction.amount
        );
    }

    Ok(())
}
