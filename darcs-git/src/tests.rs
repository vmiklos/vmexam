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
    let mut command_statuses = HashMap::new();
    command_statuses.insert("diff --quiet HEAD".to_string(), 0);
    let args: Vec<String> = vec!["darcs-git".into(), "rec".into()];
    let printed_lines = Rc::new(RefCell::new(String::new()));
    let read_line = String::new();
    let read_char = String::new();
    let ctx = TestContext {
        command_statuses,
        env_args: args,
        printed_lines,
        read_line,
        read_char,
    };

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to record anything"));
}

#[test]
fn test_record() {
    let mut command_statuses = HashMap::new();
    command_statuses.insert("diff --quiet HEAD".to_string(), 1);
    command_statuses.insert("add --patch".to_string(), 0);
    command_statuses.insert("commit -m commitmsg -e".to_string(), 0);
    let args: Vec<String> = vec!["darcs-git".into(), "rec".into()];
    let printed_lines = Rc::new(RefCell::new(String::new()));
    let read_line = "commitmsg".to_string();
    let read_char = "y".to_string();
    let ctx = TestContext {
        command_statuses,
        env_args: args,
        printed_lines,
        read_line,
        read_char,
    };

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}
