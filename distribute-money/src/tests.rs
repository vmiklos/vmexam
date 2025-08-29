/*
 * Copyright 2025 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

//! Tests the distribute_money library crate.

use super::*;
use std::collections::HashMap;

#[test]
fn test_distribute_money() {
    // Given a set of balances:
    let mut balances: HashMap<String, i64> = vec![
        ("Alice".to_string(), -97500),
        ("Bob".to_string(), 24090),
        ("Cecil".to_string(), -104090),
        ("Daniel".to_string(), 177500),
    ]
    .into_iter()
    .collect();
    let accounts: Vec<_> = balances.iter().map(|(k, v)| Account::new(k, *v)).collect();

    // When calculating transactions:
    let transactions = distribute_money(&accounts);

    // Then make sure the final balance is zero at the end:
    for transaction in transactions {
        *balances.get_mut(&transaction.from).unwrap() += transaction.amount;
        *balances.get_mut(&transaction.to).unwrap() -= transaction.amount;
    }
    for (_, v) in balances {
        assert_eq!(v, 0);
    }
}
