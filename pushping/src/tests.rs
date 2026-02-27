/*
 * Copyright 2026 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the pushping library crate.

use super::*;
use std::cell::RefCell;

/// Network implementation, for test purposes.
pub struct TestNetwork {
    url: RefCell<String>,
    data: RefCell<String>,
}

impl TestNetwork {
    pub fn new() -> Self {
        TestNetwork {
            url: RefCell::new(String::new()),
            data: RefCell::new(String::new()),
        }
    }
}

impl Network for TestNetwork {
    fn post(&self, url: String, data: String) -> anyhow::Result<()> {
        *self.url.borrow_mut() = url;
        *self.data.borrow_mut() = data;
        Ok(())
    }
}

/// Process implementation, for test purposes.
pub struct TestProcess {
    exit_code: i32,
}

impl TestProcess {
    pub fn new(exit_code: i32) -> Self {
        TestProcess { exit_code }
    }
}

impl Process for TestProcess {
    fn command_status(&self, _command: &str, _args: &[&str]) -> anyhow::Result<i32> {
        Ok(self.exit_code)
    }

    fn get_hostname(&self) -> anyhow::Result<String> {
        Ok("myhostname".to_string())
    }

    fn get_current_dir(&self) -> anyhow::Result<String> {
        Ok("/mydir".to_string())
    }
}

/// Time implementation, for test purposes.
pub struct TestTime {
    now: time::OffsetDateTime,
    counter: Rc<RefCell<i64>>,
}

impl TestTime {
    pub fn new(now: time::OffsetDateTime) -> Self {
        let counter = Rc::new(RefCell::new(0));
        TestTime { now, counter }
    }
}

impl Time for TestTime {
    fn now(&self) -> time::OffsetDateTime {
        let mut counter = self.counter.borrow_mut();
        *counter += 1;
        self.now + time::Duration::seconds(*counter)
    }
}

#[test]
fn test_run_happy() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let config_dir = root.join(&format!("{home_path}/.config")).unwrap();
    config_dir.create_dir_all().unwrap();
    let config_file = config_dir.join("pushpingrc").unwrap();
    config_file
        .create_file()
        .unwrap()
        .write_all(
            r#"access_token = "mytoken"
room_url = "https://matrix.example.com"
"#
            .as_bytes(),
        )
        .unwrap();
    let network = Rc::new(TestNetwork::new());
    let process = Rc::new(TestProcess::new(0));
    let now = time::macros::datetime!(2022-01-01 00:00:00 UTC);
    let time = Rc::new(TestTime::new(now));
    let ctx = Context::new(root, network.clone(), process, time);
    let args = vec!["pushping".into(), "echo".into(), "foo".into()];

    let exit_code = run(args, &ctx).unwrap();

    assert_eq!(exit_code, 0);
    assert_eq!(
        *network.url.borrow(),
        "https://matrix.example.com/send/m.room.message?access_token=mytoken"
    );
    let data = network.data.borrow();
    let message: Message = serde_json::from_str(&data).unwrap();
    assert_eq!(message.msgtype, "m.text");
    assert_eq!(
        message.body,
        "✓ myhostname:/mydir$ echo foo: exit code is 0, finished in 0:00:01"
    );
}

#[test]
fn test_run_failure() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let config_dir = root.join(&format!("{home_path}/.config")).unwrap();
    config_dir.create_dir_all().unwrap();
    let config_file = config_dir.join("pushpingrc").unwrap();
    config_file
        .create_file()
        .unwrap()
        .write_all(
            r#"access_token = "mytoken"
room_url = "https://matrix.example.com"
"#
            .as_bytes(),
        )
        .unwrap();
    let network = Rc::new(TestNetwork::new());
    let process = Rc::new(TestProcess::new(1));
    let now = time::macros::datetime!(2022-01-01 00:00:00 UTC);
    let time = Rc::new(TestTime::new(now));
    let ctx = Context::new(root, network.clone(), process, time);
    let args = vec!["pushping".into(), "false".into()];

    let exit_code = run(args, &ctx).unwrap();

    assert_eq!(exit_code, 1);
    assert_eq!(
        *network.url.borrow(),
        "https://matrix.example.com/send/m.room.message?access_token=mytoken"
    );
    let data = network.data.borrow();
    let message: Message = serde_json::from_str(&data).unwrap();
    assert_eq!(message.msgtype, "m.text");
    assert_eq!(
        message.body,
        "✗ myhostname:/mydir$ false: exit code is 1, finished in 0:00:01"
    );
}
