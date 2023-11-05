/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the weesearch library crate.

use super::*;

#[test]
fn test_happy() {
    let home_dir = home::home_dir().unwrap();
    let home_path = home_dir.to_string_lossy();
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let home = root.join(&home_path).unwrap();
    home.join(".local/share/weechat/logs/2020/05")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let log_file = home
        .join(".local/share/weechat/logs/2020/05/mychan1.weechatlog")
        .unwrap();
    log_file
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick	mycontent\n")
        .unwrap();

    let args: Vec<String> = vec![
        "".to_string(),
        "-d".to_string(),
        "all".to_string(),
        "mycontent".to_string(),
    ];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}
