/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

//! Tests the darcs-git library crate.

use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct TestContext {
    command_statuses: HashMap<String, i32>,
    env_args: Vec<String>,
    printed_lines: Rc<RefCell<String>>,
    read_line: String,
    read_char: String,
}

impl TestContext {
    fn new() -> Self {
        let command_statuses = HashMap::new();
        let env_args = Vec::new();
        let printed_lines = Rc::new(RefCell::new(String::new()));
        let read_line = String::new();
        let read_char = String::new();
        TestContext {
            command_statuses,
            env_args,
            printed_lines,
            read_line,
            read_char,
        }
    }
}

impl Context for TestContext {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        assert_eq!(command, "git");
        println!("TestContext::command_status: args is {:?}", args);
        let key = args.join(" ");
        Ok(self.command_statuses[&key])
    }

    fn command_output(&self, _command: &str, _args: &[&str]) -> anyhow::Result<String> {
        todo!()
    }

    fn env_args(&self) -> Vec<String> {
        self.env_args.clone()
    }

    fn print(&self, string: &str) {
        let mut printed_lines = self.printed_lines.borrow_mut();
        printed_lines.push_str(string);
    }

    fn readln(&self) -> anyhow::Result<String> {
        Ok(self.read_line.clone())
    }

    fn readch(&self) -> anyhow::Result<String> {
        Ok(self.read_char.clone())
    }
}

#[test]
fn test_record_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = [("diff --quiet HEAD".to_string(), 0)].into_iter().collect();
    let args: Vec<String> = vec!["darcs-git".into(), "rec".into()];
    ctx.env_args = args;

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to record anything"));
}

#[test]
fn test_record() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = [
        ("diff --quiet HEAD".to_string(), 1),
        ("add --patch".to_string(), 0),
        ("commit -m commitmsg -e".to_string(), 0),
    ]
    .into_iter()
    .collect();
    ctx.env_args = vec!["darcs-git".into(), "rec".into()];
    ctx.printed_lines = Rc::new(RefCell::new(String::new()));
    ctx.read_line = "commitmsg".to_string();
    ctx.read_char = "y".to_string();

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}

#[test]
fn test_revert_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = [("diff --quiet HEAD".to_string(), 0)].into_iter().collect();
    ctx.env_args = vec!["darcs-git".into(), "rev".into()];
    ctx.printed_lines = Rc::new(RefCell::new(String::new()));
    ctx.read_line = "commitmsg".to_string();
    ctx.read_char = "y".to_string();

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to revert anything"));
}

#[test]
fn test_revert() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = [
        ("diff --quiet HEAD".to_string(), 1),
        ("checkout --patch".to_string(), 0),
    ]
    .into_iter()
    .collect();
    ctx.env_args = vec!["darcs-git".into(), "rev".into()];

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_what_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = [("diff HEAD -M -C --exit-code".to_string(), 0)]
        .into_iter()
        .collect();
    ctx.env_args = vec!["darcs-git".into(), "what".into()];

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("No changes"));
}
