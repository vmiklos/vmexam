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

/// Time implementation, for test purposes.
pub struct TestTime {
    now: time::OffsetDateTime,
}

impl TestTime {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        let date = time::Date::from_calendar_date(
            year,
            time::Month::try_from(month as u8).unwrap(),
            day as u8,
        )
        .unwrap()
        .midnight();
        let now = date.assume_utc();
        TestTime { now }
    }
}

impl Time for TestTime {
    fn now(&self) -> time::OffsetDateTime {
        self.now
    }
}

#[test]
fn test_regex() {
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
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "my.*ent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_fixed() {
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
        .write_all(b"2020-05-10 19:34:33	mynick	+36\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "-F".to_string(), "+36".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	+36\n"
    );
}

#[test]
fn test_fixed_ignore_case() {
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
        .write_all(b"2020-05-10 19:34:33	mynick	FOO\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec![
        "".to_string(),
        "-F".to_string(),
        "-i".to_string(),
        "foo".to_string(),
    ];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	FOO\n"
    );
}

#[test]
fn test_from() {
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
        .write_all(
            b"2020-05-10 19:34:33	mynick1	mycontent
2020-05-10 19:34:33	mynick2	mycontent\n",
        )
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "-f".to_string(), "mynick1".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_channel() {
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
        .write_all(b"2020-05-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    let log_file2 = home
        .join(".local/share/weechat/logs/2020/05/mychan2.weechatlog")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "-c".to_string(), "mychan1".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_date() {
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
        .write_all(b"2020-05-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    home.join(".local/share/weechat/logs/2020/06")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let log_file2 = home
        .join(".local/share/weechat/logs/2020/06/mychan1.weechatlog")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-06-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "-d".to_string(), "2020-06".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-06-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_date_all() {
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
        .write_all(b"2020-05-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    home.join(".local/share/weechat/logs/2020/06")
        .unwrap()
        .create_dir_all()
        .unwrap();
    let log_file2 = home
        .join(".local/share/weechat/logs/2020/06/mychan1.weechatlog")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-06-10 19:34:33	mynick1	mycontent\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "-d".to_string(), "all".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent
mychan1.weechatlog:2020-06-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_file_under_logs() {
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
    let log_file2 = home
        .join(".local/share/weechat/logs/away.weechatlog")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick	myaway\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "my.*ent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_file_under_year() {
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
    let log_file2 = home
        .join(".local/share/weechat/logs/2020/away.weechatlog")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick	myaway\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "my.*ent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_no_extension() {
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
    // No extension.
    let log_file2 = home.join(".local/share/weechat/logs/2020/05/away").unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick	myaway\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "my.*ent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_bad_extension() {
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
    // Bad extension.
    let log_file2 = home
        .join(".local/share/weechat/logs/2020/05/away.log")
        .unwrap();
    log_file2
        .create_file()
        .unwrap()
        .write_all(b"2020-05-10 19:34:33	mynick	myaway\n")
        .unwrap();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "my.*ent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    let buf_string = String::from_utf8(buf_vec).unwrap();
    assert_eq!(
        buf_string,
        "mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_regex_no_match() {
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
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "yourcontent".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 0);

    let buf_vec = buf.into_inner();
    assert!(buf_vec.is_empty());
}

#[test]
fn test_regex_bad() {
    let root: vfs::VfsPath = vfs::MemoryFS::new().into();
    let time = TestTime::new(2020, 5, 10);

    let args: Vec<String> = vec!["".to_string(), "+36".to_string()];
    let mut buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());

    assert_eq!(main(args, &mut buf, &root, &time), 1);
}
