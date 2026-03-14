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

/// Time implementation, for test purposes.
pub struct TestTime {
    now: time::OffsetDateTime,
}

impl TestTime {
    pub fn new(now: time::OffsetDateTime) -> Self {
        TestTime { now }
    }
}

impl Time for TestTime {
    fn now(&self) -> time::OffsetDateTime {
        self.now
    }
}

#[test]
fn test_run_happy() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let log_dir = "/logs";
    root.join(log_dir).unwrap().create_dir_all().unwrap();
    let now = time::macros::datetime!(2026-03-13 23:45:00 UTC);
    let time = Rc::new(TestTime::new(now));
    let ctx = Context::new(root.clone(), time);
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
    assert_eq!(log_content, "[2026-03-13 23:45:00] bar\n");
    assert_eq!(stdout, b"{}\n");
}

#[test]
fn test_run_multiple_entries() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let log_dir = "/logs";
    root.join(log_dir).unwrap().create_dir_all().unwrap();
    let now = time::macros::datetime!(2026-03-13 23:45:00 UTC);
    let time = Rc::new(TestTime::new(now));
    let ctx = Context::new(root.clone(), time);
    let args = vec![
        "json-logger".into(),
        "--key".into(),
        "foo".into(),
        "--log-dir".into(),
        log_dir.into(),
    ];
    let mut stdin = std::io::Cursor::new(r#"{"foo": "bar"}{"foo": "baz"}"#);
    let mut stdout = Vec::new();

    run(args, &ctx, &mut stdin, &mut stdout).unwrap();

    let cwd = std::env::current_dir().unwrap();
    let filename = cwd.to_string_lossy().replace("/", "-");
    let log_path = root.join(log_dir).unwrap().join(&filename).unwrap();
    let mut log_file = log_path.open_file().unwrap();
    let mut log_content = String::new();
    log_file.read_to_string(&mut log_content).unwrap();
    let expected = "[2026-03-13 23:45:00] bar\n[2026-03-13 23:45:00] baz\n";
    assert_eq!(log_content, expected);
}
