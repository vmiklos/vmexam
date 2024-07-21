/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

//! Tests the darcs-git library crate.

use super::*;
use std::collections::HashMap;

struct TestContext {
    command_statuses: HashMap<String, i32>,
    env_args: Vec<String>,
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
}

#[test]
fn test_record_no_changes() {
    let mut command_statuses = HashMap::new();
    command_statuses.insert("diff --quiet HEAD".to_string(), 0);
    let args: Vec<String> = vec!["darcs-git".into(), "rec".into()];
    let ctx = TestContext {
        command_statuses,
        env_args: args,
    };

    main(&ctx).unwrap();

    // TODO assert the output
}
