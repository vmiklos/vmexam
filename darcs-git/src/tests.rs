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
}

impl Context for TestContext {
    fn command_status(&self, command: &str, args: &[&str]) -> anyhow::Result<i32> {
        assert_eq!(command, "git");
        println!("debug, TestContext::command_status: args is {:?}", args);
        let key = args.join(" ");
        Ok(self.command_statuses[&key])
    }

    fn command_output(&self, _command: &str, _args: &[&str]) -> anyhow::Result<String> {
        todo!()
    }

    fn env_args(&self) -> Vec<String> {
        self.env_args.clone()
    }

    fn println(&self, string: &str) {
        let mut printed_lines = self.printed_lines.borrow_mut();
        printed_lines.push_str(string);
    }
}

#[test]
fn test_record_no_changes() {
    let mut command_statuses = HashMap::new();
    command_statuses.insert("diff --quiet HEAD".to_string(), 0);
    let args: Vec<String> = vec!["darcs-git".into(), "rec".into()];
    let printed_lines = Rc::new(RefCell::new(String::new()));
    let ctx = TestContext {
        command_statuses,
        env_args: args,
        printed_lines,
    };

    main(&ctx).unwrap();

    let printed_lines = ctx.printed_lines.borrow();
    assert!(printed_lines.contains("don't want to record anything"));
}
