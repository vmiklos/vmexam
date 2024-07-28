/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

//! Tests the darcs-git library crate.

use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;

struct TestContext {
    /// Joined cmdline, exit status.
    command_statuses: RefCell<VecDeque<(String, i32)>>,
    /// Joined cmdline, output.
    command_outputs: RefCell<VecDeque<(String, String)>>,
    env_args: Vec<String>,
    printed_lines: RefCell<String>,
    read_line: String,
    read_chars: RefCell<VecDeque<String>>,
}

impl TestContext {
    fn new() -> Self {
        let command_statuses = RefCell::new(VecDeque::new());
        let command_outputs = RefCell::new(VecDeque::new());
        let env_args = Vec::new();
        let printed_lines = RefCell::new(String::new());
        let read_line = String::new();
        let read_chars = RefCell::new(VecDeque::new());
        TestContext {
            command_statuses,
            command_outputs,
            env_args,
            printed_lines,
            read_line,
            read_chars,
        }
    }

    fn set_env_args(&mut self, env_args: &[&str]) {
        let mut v: Vec<String> = vec!["darcs-git".into()];
        v.append(&mut env_args.into_iter().map(|i| i.to_string()).collect());
        self.env_args = v;
    }
}

impl Context for TestContext {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        assert_eq!(command, "git");
        println!("TestContext::command_status: args is {:?}", args);
        let cmdline = args.join(" ");
        let mut command_statuses = self.command_statuses.borrow_mut();
        let command_status = command_statuses.pop_front().unwrap();
        assert_eq!(command_status.0, cmdline);
        Ok(command_status.1)
    }

    fn command_output(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        assert_eq!(command, "git");
        println!("TestContext::command_output: args is {:?}", args);
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

    fn readln(&self) -> anyhow::Result<String> {
        Ok(self.read_line.clone())
    }

    fn readch(&self) -> anyhow::Result<String> {
        let mut read_chars = self.read_chars.borrow_mut();
        let read_char = read_chars.pop_front().unwrap();
        Ok(read_char.clone())
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let command_statuses = self.command_statuses.borrow();
        assert!(command_statuses.is_empty());
        let command_outputs = self.command_outputs.borrow();
        assert!(command_outputs.is_empty());
        let read_chars = self.read_chars.borrow_mut();
        assert!(read_chars.is_empty());
    }
}

#[test]
fn test_record_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([("diff --quiet HEAD".to_string(), 0)]));
    ctx.set_env_args(&["rec"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to record anything"));
}

#[test]
fn test_record() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("add --patch".to_string(), 0),
        ("commit -m commitmsg -e".to_string(), 0),
    ]));
    ctx.set_env_args(&["rec"]);
    ctx.read_line = "commitmsg".to_string();
    ctx.read_chars = RefCell::new(VecDeque::from(["y".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}

#[test]
fn test_record_files() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("add --patch file1".to_string(), 0),
        ("commit -m commitmsg -e".to_string(), 0),
    ]));
    ctx.set_env_args(&["rec", "file1"]);
    ctx.read_line = "commitmsg".to_string();
    ctx.read_chars = RefCell::new(VecDeque::from(["y".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}

/// Tests the case when record() quits because the answer to "do you want a long comment" is not y
/// or n.
#[test]
fn test_record_quit() {
    let mut ctx = TestContext::new();
    // Note the lack of 'commit' here.
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("add --patch".to_string(), 0),
    ]));
    ctx.set_env_args(&["rec"]);
    ctx.read_line = "commitmsg".to_string();
    ctx.read_chars = RefCell::new(VecDeque::from(["q".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}

/// Tests the case when we try again because the answer to "do you want a long comment" is not y, n
/// or q.
#[test]
fn test_record_try_again() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("add --patch".to_string(), 0),
        ("commit -m commitmsg -e".to_string(), 0),
    ]));
    ctx.set_env_args(&["rec"]);
    ctx.read_line = "commitmsg".to_string();
    ctx.read_chars = RefCell::new(VecDeque::from(["x".to_string(), "y".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("commit message?"));
    assert!(printed_lines.contains("long comment?"));
}

#[test]
fn test_revert_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([("diff --quiet HEAD".to_string(), 0)]));
    ctx.set_env_args(&["rev"]);
    ctx.read_line = "commitmsg".to_string();

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to revert anything"));
}

#[test]
fn test_revert() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("checkout --patch".to_string(), 0),
    ]));
    ctx.set_env_args(&["rev"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_revert_files() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("diff --quiet HEAD".to_string(), 1),
        ("checkout --patch file1".to_string(), 0),
    ]));
    ctx.set_env_args(&["rev", "file1"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_what_no_changes() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([(
        "diff HEAD -M -C --exit-code".to_string(),
        0,
    )]));
    ctx.set_env_args(&["what"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("No changes"));
}

#[test]
fn test_what() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([(
        "diff HEAD -M -C --exit-code".to_string(),
        1,
    )]));
    ctx.set_env_args(&["what"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_what_files() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([(
        "diff HEAD -M -C --exit-code file1".to_string(),
        1,
    )]));
    ctx.set_env_args(&["what", "file1"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_what_summary() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([(
        "diff HEAD -M -C --exit-code --name-status".to_string(),
        1,
    )]));
    ctx.set_env_args(&["what", "-s"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.is_empty());
}

#[test]
fn test_push_nothing_to_push() {
    let mut ctx = TestContext::new();
    ctx.command_outputs = RefCell::new(VecDeque::from([(
        "log HEAD@{upstream}..".to_string(),
        "".to_string(),
    )]));
    ctx.set_env_args(&["push"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("No recorded local changes to push"));
}

#[test]
fn test_push() {
    let mut ctx = TestContext::new();
    ctx.command_outputs = RefCell::new(VecDeque::from([(
        "log HEAD@{upstream}..".to_string(),
        "log-output".to_string(),
    )]));
    ctx.command_statuses = RefCell::new(VecDeque::from([("push".to_string(), 0)]));
    ctx.set_env_args(&["push"]);
    ctx.read_chars = RefCell::new(VecDeque::from(["y".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("push these patches?"));
}

#[test]
fn test_push_cancel() {
    let mut ctx = TestContext::new();
    ctx.command_outputs = RefCell::new(VecDeque::from([(
        "log HEAD@{upstream}..".to_string(),
        "log-output".to_string(),
    )]));
    // ctx.command_statuses is empty, no 'push' is expected.
    ctx.set_env_args(&["push"]);
    ctx.read_chars = RefCell::new(VecDeque::from(["n".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("push these patches?"));
}

#[test]
fn test_push_try_again() {
    let mut ctx = TestContext::new();
    ctx.command_outputs = RefCell::new(VecDeque::from([(
        "log HEAD@{upstream}..".to_string(),
        "log-output".to_string(),
    )]));
    ctx.command_statuses = RefCell::new(VecDeque::from([("push".to_string(), 0)]));
    ctx.set_env_args(&["push"]);
    ctx.read_chars = RefCell::new(VecDeque::from(["x".to_string(), "y".to_string()]));

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("push these patches?"));
}

#[test]
fn test_unrec() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("log -1".to_string(), 0),
        ("reset --quiet HEAD^".to_string(), 0),
    ]));
    ctx.read_chars = RefCell::new(VecDeque::from(["y".to_string()]));
    ctx.set_env_args(&["unrec"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("unrecording"));
}

#[test]
fn test_unpull() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([
        ("log -1".to_string(), 0),
        ("reset --hard HEAD^".to_string(), 0),
    ]));
    ctx.read_chars = RefCell::new(VecDeque::from(["y".to_string()]));
    ctx.set_env_args(&["unpull"]);

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("unpulling"));
}

#[test]
fn test_checked_run_fails() {
    let mut ctx = TestContext::new();
    ctx.command_statuses = RefCell::new(VecDeque::from([("false".to_string(), 1)]));

    let ret = checked_run(&ctx, "git", &["false"]);

    assert!(ret.is_err());
}
