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

struct TestContext {
    args: Vec<String>,
    buf: std::io::Cursor<Vec<u8>>,
    root: vfs::VfsPath,
    home: vfs::VfsPath,
    time: TestTime,
}

impl TestContext {
    pub fn new(args_slice: &[&str]) -> Self {
        let mut args: Vec<String> = vec!["".to_string()];
        args.append(&mut args_slice.iter().map(|i| i.to_string()).collect());
        let buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
        let home_dir = home::home_dir().unwrap();
        let home_path = home_dir.to_string_lossy();
        let root: vfs::VfsPath = vfs::MemoryFS::new().into();
        let home = root.join(&home_path).unwrap();
        let time = TestTime::new(2020, 5, 10);
        TestContext {
            args,
            buf,
            root,
            home,
            time,
        }
    }

    pub fn into_buf_string(&self) -> String {
        let home_dir = home::home_dir().unwrap();
        let home_path = home_dir.to_string_lossy().to_string() + "/";
        let buf_vec = self.buf.clone().into_inner();
        String::from_utf8(buf_vec).unwrap().replace(&home_path, "")
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn create_dir_all(&self, path: &str) {
        self.home.join(path).unwrap().create_dir_all().unwrap();
    }

    pub fn file_write_all(&self, path: &str, bytes: &[u8]) {
        let file = self.home.join(path).unwrap();
        file.create_file().unwrap().write_all(bytes).unwrap();
    }
}

#[test]
fn test_regex() {
    let mut ctx = TestContext::new(&["my.*ent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_fixed() {
    let mut ctx = TestContext::new(&["-F", "+36"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	+36\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	+36\n"
    );
}

#[test]
fn test_fixed_ignore_case() {
    let mut ctx = TestContext::new(&["-F", "-i", "foo"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	FOO\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	FOO\n"
    );
}

#[test]
fn test_from() {
    let mut ctx = TestContext::new(&["-f", "mynick1"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick1	mycontent
2020-05-10 19:34:33	mynick2	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_channel() {
    let mut ctx = TestContext::new(&["-c", "mychan1"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick1	mycontent\n",
    );
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan2.weechatlog",
        b"2020-05-10 19:34:33	mynick1	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_date() {
    let mut ctx = TestContext::new(&["-d", "2020-06"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick1	mycontent\n",
    );
    ctx.create_dir_all(".local/share/weechat/logs/2020/06");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/06/mychan1.weechatlog",
        b"2020-06-10 19:34:33	mynick1	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/06/mychan1.weechatlog:2020-06-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_date_all() {
    let mut ctx = TestContext::new(&["-d", "all"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick1	mycontent\n",
    );
    ctx.create_dir_all(".local/share/weechat/logs/2020/06");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/06/mychan1.weechatlog",
        b"2020-06-10 19:34:33	mynick1	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick1	mycontent
.local/share/weechat/logs/2020/06/mychan1.weechatlog:2020-06-10 19:34:33	mynick1	mycontent\n"
    );
}

#[test]
fn test_file_under_logs() {
    let mut ctx = TestContext::new(&["my.*ent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );
    ctx.file_write_all(
        ".local/share/weechat/logs/away.weechatlog",
        b"2020-05-10 19:34:33	mynick	myaway\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_file_under_year() {
    let mut ctx = TestContext::new(&["my.*ent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/away.weechatlog",
        b"2020-05-10 19:34:33	mynick	myaway\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_no_extension() {
    let mut ctx = TestContext::new(&["my.*ent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );
    // No extension.
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/away",
        b"2020-05-10 19:34:33	mynick	myaway\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_bad_extension() {
    let mut ctx = TestContext::new(&["my.*ent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );
    // Bad extension.
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/away.log",
        b"2020-05-10 19:34:33	mynick	myaway\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	mycontent\n"
    );
}

#[test]
fn test_regex_no_match() {
    let mut ctx = TestContext::new(&["yourcontent"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        b"2020-05-10 19:34:33	mynick	mycontent\n",
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert!(buf.is_empty());
}

#[test]
fn test_regex_bad() {
    let mut ctx = TestContext::new(&["+36"]);

    let ret = main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time);

    assert_eq!(ret, 1);
}

#[test]
fn test_transliterate() {
    let mut ctx = TestContext::new(&["--transliterate", "he"]);
    ctx.create_dir_all(".local/share/weechat/logs/2020/05");
    ctx.file_write_all(
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog",
        "2020-05-10 19:34:33	mynick	hé\n".as_bytes(),
    );

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.root, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        ".local/share/weechat/logs/2020/05/mychan1.weechatlog:2020-05-10 19:34:33	mynick	hé\n"
    );
}
