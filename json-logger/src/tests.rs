/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the json-logger library crate.

use super::*;
use std::io::Read as _;

#[test]
fn test_run_happy() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let log_dir = "/logs";
    root.join(log_dir).unwrap().create_dir_all().unwrap();
    let ctx = Context::new(root.clone());
    let args = vec![
        "json-logger".into(),
        "--key".into(),
        "foo".into(),
        "--log-dir".into(),
        log_dir.into(),
        "--print-empty".into(),
    ];
    let mut stdin = std::io::Cursor::new(r#"{"foo": "bar"}"#);
    let mut stdout = Vec::new();

    run(args, &ctx, &mut stdin, &mut stdout).unwrap();

    let cwd = std::env::current_dir().unwrap();
    let filename = cwd.to_string_lossy().replace("/", "-");
    let log_path = root.join(log_dir).unwrap().join(&filename).unwrap();
    let mut log_file = log_path.open_file().unwrap();
    let mut log_content = String::new();
    log_file.read_to_string(&mut log_content).unwrap();
    assert!(log_content.contains("bar"));
    assert_eq!(stdout, b"{}\n");
}
