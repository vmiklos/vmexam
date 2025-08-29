/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! A simple money distribution library, similar to splitwise.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

/// A person with a name and a given amount of money contribution.
///
/// Positive balance represents a creditor, negative balance means a debtor.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: &str, balance: i64) -> Self {
        let owner = owner.to_string();
        Account { owner, balance }
    }
}

/// A suggested move of a certain amount of money between accounts.
pub struct Transaction {
    /// Source account.
    pub from: String,
    /// Target account.
    pub to: String,
    /// The money to transfer.
    pub amount: i64,
}

impl Transaction {
    fn new(from: &str, to: &str, amount: i64) -> Self {
        let from = from.to_string();
        let to = to.to_string();
        Transaction { from, to, amount }
    }
}

/// Generates a list of transactions, so the balance on accounts will be zero.
pub fn distribute_money(accounts: &[Account]) -> Vec<Transaction> {
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

#[cfg(test)]
mod tests;
