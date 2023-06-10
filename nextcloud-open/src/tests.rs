/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the nextcloud_open library crate.

use super::*;
use std::cell::RefCell;

/// Network implementation, for test purposes.
pub struct TestNetwork {
    open_browsers: Rc<RefCell<Vec<url::Url>>>,
}

impl TestNetwork {
    pub fn new() -> Self {
        let open_browsers = Rc::new(RefCell::new(Vec::new()));
        TestNetwork { open_browsers }
    }
}

impl Network for TestNetwork {
    fn open_browser(&self, url: &url::Url) {
        self.open_browsers.borrow_mut().push(url.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[test]
fn test_happy() {
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    root.join(".config/Nextcloud")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let config_file = root.join(".config/Nextcloud/nextcloud.cfg").unwrap();
    config_file
        .create_file()
        .unwrap()
        .write_all(
            format!(
                "[Accounts]\n\
0\\Folders\\1\\localPath={home_path}/Nextcloud-Example/\n\
0\\url=https://nextcloud.example.com\n\
version=2"
            )
            .as_bytes(),
        )
        .unwrap();
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new());
    let ctx = Context::new(root, network);
    let input =
        std::path::PathBuf::from(format!("{home_path}/Nextcloud-Example/my dir/my file.md"));

    nextcloud_open(&ctx, &input).unwrap();

    let network = ctx.network.as_any().downcast_ref::<TestNetwork>().unwrap();
    let open_browsers = network.open_browsers.borrow_mut();
    assert_eq!(open_browsers.len(), 1);
    let expected = "https://nextcloud.example.com/apps/files/?dir=/my%20dir/&scrollto=my%20file.md";
    assert_eq!(open_browsers[0].to_string(), expected);
}

/// Tests what happens when the nextcloud config file can't be parsed.
#[test]
fn test_config_read_error() {
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    root.join(".config/Nextcloud")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let config_file = root.join(".config/Nextcloud/nextcloud.cfg").unwrap();
    // Opening bracket for section name but no closing bracket.
    config_file
        .create_file()
        .unwrap()
        .write_all(format!("[Invalid").as_bytes())
        .unwrap();
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new());
    let ctx = Context::new(root, network);
    let input =
        std::path::PathBuf::from(format!("{home_path}/Nextcloud-Example/my dir/my file.md"));

    let ret = nextcloud_open(&ctx, &input);

    assert_eq!(ret.is_err(), true);
}
