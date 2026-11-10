/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the git-tzstats library crate.

use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use time_tz::Offset as _;
use time_tz::TimeZone as _;

/// Time implementation, backed by an explicit timezone.
struct TestTime {
    timezone: &'static time_tz::Tz,
}

impl TestTime {
    fn new(name: &str) -> Self {
        let timezone = time_tz::timezones::get_by_name(name).unwrap();
        TestTime { timezone }
    }
}

impl Time for TestTime {
    fn local_offset(&self, date_time: time::OffsetDateTime) -> anyhow::Result<time::UtcOffset> {
        Ok(self.timezone.get_offset_utc(&date_time).to_utc())
    }
}

struct TestContext {
    command_outputs: RefCell<VecDeque<(String, String)>>,
    env_args: Vec<String>,
    printed_lines: RefCell<String>,
}

impl TestContext {
    fn new() -> Self {
        let command_outputs = RefCell::new(VecDeque::new());
        let env_args = Vec::new();
        let printed_lines = RefCell::new(String::new());
        TestContext {
            command_outputs,
            env_args,
            printed_lines,
        }
    }

    fn set_env_args(&mut self, env_args: &[&str]) {
        let mut v: Vec<String> = vec!["git-tzstats".into()];
        v.append(&mut env_args.iter().map(|i| i.to_string()).collect());
        self.env_args = v;
    }

    fn set_command_outputs(&mut self, command_outputs: &[(&str, &str)]) {
        let v: Vec<(String, String)> = command_outputs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.command_outputs = RefCell::new(VecDeque::from(v));
    }

    fn buf_string(&self) -> String {
        self.printed_lines.borrow().clone()
    }
}

impl Context for TestContext {
    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        assert_eq!(command, "git");
        let cmdline = args.join(" ");
        let mut command_outputs = self.command_outputs.borrow_mut();
        let command_output = command_outputs.pop_front().unwrap();
        assert_eq!(command_output.0, cmdline);
        Ok(command_output.1)
    }

    fn env_args(&self) -> Vec<String> {
        self.env_args.clone()
    }

    fn print(&self, string: &str) {
        let mut printed_lines = self.printed_lines.borrow_mut();
        printed_lines.push_str(string);
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let command_outputs = self.command_outputs.borrow();
        assert!(command_outputs.is_empty());
    }
}

fn format_output(commits: &[(&str, &str)]) -> String {
    commits
        .iter()
        .map(|(author, date)| format!("{author}\x1f{date}\x1e\n"))
        .collect()
}

#[test]
fn test_travel() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2018-01-03T09:00:00+01:00"),
        ("Alice", "2018-03-26T09:00:00+02:00"),
        ("Alice", "2018-06-21T09:00:00+01:00"),
        ("Alice", "2018-06-22T09:00:00+02:00"),
    ]);
    ctx.set_command_outputs(&[(
        "log --pretty=format:%an%x1f%aI%x1e --author=vmiklos --since=2018-01-01 --until=2018-12-31",
        &output,
    )]);
    ctx.set_env_args(&[
        "--author=vmiklos",
        "--since=2018-01-01",
        "--until=2018-12-31",
    ]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(
        ctx.buf_string(),
        "Alice traveled to +0100 on 2018-06-21 and returned to +0200 on 2018-06-22.\n"
    );
}

#[test]
fn test_dst_change() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2018-01-03T09:00:00+01:00"),
        ("Alice", "2018-03-26T09:00:00+02:00"),
        ("Alice", "2018-10-29T09:00:00+01:00"),
    ]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(ctx.buf_string(), "");
}

#[test]
fn test_dst_grace() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2023-01-02T09:00:00+01:00"),
        ("Alice", "2023-03-24T09:00:00+01:00"),
        ("Alice", "2023-03-27T09:00:00+02:00"),
        ("Alice", "2023-03-28T22:00:00+01:00"),
        ("Alice", "2023-03-29T22:00:00+01:00"),
        ("Alice", "2023-03-31T09:00:00+02:00"),
    ]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(ctx.buf_string(), "");
}

#[test]
fn test_mixed_day() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2023-01-02T09:00:00+01:00"),
        ("Alice", "2023-06-21T09:00:00+02:00"),
        ("Alice", "2023-06-21T21:00:00+01:00"),
        ("Alice", "2023-06-22T09:00:00+02:00"),
    ]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(ctx.buf_string(), "");
}

#[test]
fn test_travel_after_dst_grace() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2024-01-02T09:00:00+01:00"),
        ("Alice", "2024-04-09T09:00:00+01:00"),
        ("Alice", "2024-04-12T09:00:00+02:00"),
    ]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(
        ctx.buf_string(),
        "Alice traveled to +0100 on 2024-04-09 and returned to +0200 on 2024-04-12.\n"
    );
}

#[test]
fn test_invalid_date() {
    let mut ctx = TestContext::new();
    let output = format_output(&[("Alice", "not-a-date")]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    let result = main(&ctx, &time);

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("failed to parse author date")
    );
}

#[test]
fn test_multiple_authors() {
    let mut ctx = TestContext::new();
    let output = format_output(&[
        ("Alice", "2018-01-03T09:00:00+01:00"),
        ("Alice", "2018-06-21T09:00:00+01:00"),
        ("Alice", "2018-06-22T09:00:00+02:00"),
        ("Bob", "2018-01-03T09:00:00+01:00"),
        ("Bob", "2018-06-21T09:00:00+02:00"),
    ]);
    ctx.set_command_outputs(&[("log --pretty=format:%an%x1f%aI%x1e", &output)]);
    ctx.set_env_args(&[]);
    let time = TestTime::new("Europe/London");

    main(&ctx, &time).unwrap();

    assert_eq!(
        ctx.buf_string(),
        "Alice traveled to +0100 on 2018-06-21 and returned to +0200 on 2018-06-22.\n"
    );
}
