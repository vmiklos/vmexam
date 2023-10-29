/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

//! Tests the tpconv library crate.

use super::*;

#[test]
fn test_happy() {
    let args: Vec<String> = vec![
        "".to_string(),
        "2".to_string(),
        "inches".to_string(),
        "in".to_string(),
        "cm".to_string(),
    ];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(buf_string, "5.08\n");
}

#[test]
fn test_invalid_float_literal() {
    let args: Vec<String> = vec![
        "".to_string(),
        "x".to_string(),
        "inches".to_string(),
        "in".to_string(),
        "cm".to_string(),
    ];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf), 1);
}
