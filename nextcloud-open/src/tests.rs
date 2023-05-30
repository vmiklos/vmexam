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
0\\url=https://nextcloud.example.com"
            )
            .as_bytes(),
        )
        .unwrap();
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new());
    let ctx = Context::new(root, network);
    let input =
        std::path::PathBuf::from(format!("{home_path}/Nextcloud-Example/my dir/my file.md"));

    nextcloud_open(&ctx, &input).unwrap();
}
