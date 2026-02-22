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

/// Contains info about how to patch out one URL.
#[derive(Clone)]
pub struct URLRoute {
    /// HTTP user
    user: String,
    /// HTTP password
    password: String,
    /// HTTP method
    method: String,
    /// The request URL
    url: String,
    /// Path of expected POST data, empty for GET
    data_path: String,
    /// Path of expected result data
    result_path: String,
}

impl URLRoute {
    pub fn new(
        user: &str,
        password: &str,
        method: &str,
        url: &str,
        data_path: &str,
        result_path: &str,
    ) -> Self {
        URLRoute {
            user: user.into(),
            password: password.into(),
            method: method.into(),
            url: url.into(),
            data_path: data_path.into(),
            result_path: result_path.into(),
        }
    }
}

/// Network implementation, for test purposes.
pub struct TestNetwork {
    open_browsers: Rc<RefCell<Vec<url::Url>>>,
    routes: Rc<RefCell<Vec<URLRoute>>>,
}

impl TestNetwork {
    pub fn new(routes: &[URLRoute]) -> Self {
        let open_browsers = Rc::new(RefCell::new(Vec::new()));
        let routes = Rc::new(RefCell::new(routes.to_vec()));
        TestNetwork {
            open_browsers,
            routes,
        }
    }
}

impl Network for TestNetwork {
    fn open_browser(&self, url: &url::Url) {
        self.open_browsers.borrow_mut().push(url.clone());
    }

    fn send_request(
        &self,
        user: &str,
        password: &str,
        method: &str,
        url: &str,
        data: &str,
    ) -> anyhow::Result<String> {
        let mut ret: String = "".into();
        let mut remove: Option<usize> = None;
        let mut locked_routes = self.routes.borrow_mut();
        for (index, route) in locked_routes.iter().enumerate() {
            // In the future: if not, continue.
            assert_eq!(url, route.url);

            // In the future: only if it's not empty.
            assert_eq!(user, route.user);

            // In the future: only if it's not empty.
            assert_eq!(password, route.password);

            assert_eq!(method, route.method);

            // In the future: only if it's not empty.
            let expected = std::fs::read_to_string(&route.data_path)?;
            assert_eq!(data, expected);

            ret = std::fs::read_to_string(&route.result_path)?;
            remove = Some(index);
            break;
        }

        // Allow specifying multiple results for the same URL.
        locked_routes.remove(remove.unwrap());
        Ok(ret)
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
    let home = root.join(&home_path).unwrap();
    home.join(".config/Nextcloud")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let config_file = home.join(".config/Nextcloud/nextcloud.cfg").unwrap();
    config_file
        .create_file()
        .unwrap()
        .write_all(
            format!(
                "[Accounts]\n\
0\\Folders\\1\\localPath={home_path}/Nextcloud-Example/\n\
0\\url=https://nextcloud.example.com\n\
version=4\n\
0\\keyWithNoValue"
            )
            .as_bytes(),
        )
        .unwrap();
    let credentials = home.join(".config/nextcloud-openrc").unwrap();
    credentials
        .create_file()
        .unwrap()
        .write_all(
            r#"[credentials."https://nextcloud.example.com"]
user = "myuser"
principal = "myprincipal"
password = "mypassword"
"#
            .to_string()
            .as_bytes(),
        )
        .unwrap();
    let routes = vec![URLRoute::new(
        /*user=*/ "myuser",
        /*password=*/ "mypassword",
        /*method=*/ "PROPFIND",
        /*url=*/
        "https://nextcloud.example.com/remote.php/dav/files/myprincipal/my%20dir/my%20file.md",
        /*data_path=*/ "src/fixtures/network/webdav-input.xml",
        /*result_path=*/ "src/fixtures/network/webdav-output.xml",
    )];
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new(&routes));
    let input = root
        .join(format!("{home_path}/Nextcloud-Example/my dir/my file.md"))
        .unwrap();
    let ctx = Context::new(root, network);

    nextcloud_open(&ctx, &[input]).unwrap();

    let network = ctx.network.as_any().downcast_ref::<TestNetwork>().unwrap();
    let open_browsers = network.open_browsers.borrow_mut();
    assert_eq!(open_browsers.len(), 1);
    let expected =
        "https://nextcloud.example.com/index.php/apps/files/files/42?dir=/my%20dir&openfile=true";
    assert_eq!(open_browsers[0].to_string(), expected);
}

/// Tests what happens when the nextcloud config file can't be parsed.
#[test]
fn test_config_read_error() {
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let home = root.join(&home_path).unwrap();
    home.join(".config/Nextcloud")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let config_file = home.join(".config/Nextcloud/nextcloud.cfg").unwrap();
    // Opening bracket for section name but no closing bracket.
    config_file
        .create_file()
        .unwrap()
        .write_all("[Invalid".to_string().as_bytes())
        .unwrap();
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new(&[]));
    let input = root
        .join(format!("{home_path}/Nextcloud-Example/my dir/my file.md"))
        .unwrap();
    let ctx = Context::new(root, network);

    let ret = nextcloud_open(&ctx, &[input]);

    assert!(ret.is_err());
}

/// Tests the case when the input is a directory, not a file.
#[test]
fn test_input_is_dir() {
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let home = root.join(&home_path).unwrap();
    home.join(".config/Nextcloud")
        .unwrap()
        .create_dir_all()
        .unwrap();
    home.join("Nextcloud-Example/my dir")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let config_file = home.join(".config/Nextcloud/nextcloud.cfg").unwrap();
    config_file
        .create_file()
        .unwrap()
        .write_all(
            format!(
                "[Accounts]\n\
0\\Folders\\1\\localPath={home_path}/Nextcloud-Example/\n\
0\\url=https://nextcloud.example.com\n\
version=4\n\
0\\keyWithNoValue"
            )
            .as_bytes(),
        )
        .unwrap();
    let network: Rc<dyn Network> = Rc::new(TestNetwork::new(&[]));
    let input = root
        .join(format!("{home_path}/Nextcloud-Example/my dir"))
        .unwrap();
    let ctx = Context::new(root, network);

    nextcloud_open(&ctx, &[input]).unwrap();

    let network = ctx.network.as_any().downcast_ref::<TestNetwork>().unwrap();
    let open_browsers = network.open_browsers.borrow_mut();
    assert_eq!(open_browsers.len(), 1);
    let expected = "https://nextcloud.example.com/index.php/apps/files/?dir=/my%20dir/";
    assert_eq!(open_browsers[0].to_string(), expected);
}
