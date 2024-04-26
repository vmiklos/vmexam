/*
 * Copyright 2023 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
*/

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Tests the ics2txt library crate.

use super::*;

/// Time implementation, for test purposes.
pub struct TestTime {
    current_local_offset: time::UtcOffset,
}

impl TestTime {
    pub fn new() -> Self {
        let current_local_offset = time::UtcOffset::from_hms(1, 0, 0).unwrap();
        TestTime {
            current_local_offset,
        }
    }
}

impl Time for TestTime {
    fn current_local_offset(&self) -> anyhow::Result<time::UtcOffset> {
        Ok(self.current_local_offset)
    }
}

struct TestContext {
    args: Vec<String>,
    buf: std::io::Cursor<Vec<u8>>,
    time: TestTime,
}

impl TestContext {
    pub fn new(args_slice: &[&str]) -> Self {
        let mut args: Vec<String> = vec!["".to_string()];
        args.append(&mut args_slice.iter().map(|i| i.to_string()).collect());
        let buf: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
        let time = TestTime::new();
        TestContext { args, buf, time }
    }

    pub fn into_buf_string(&self) -> String {
        let buf_vec = self.buf.clone().into_inner();
        String::from_utf8(buf_vec).unwrap()
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }
}

#[test]
fn test_different_tz() {
    let mut ctx = TestContext::new(&["src/fixtures/different-tz.ics"]);

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        r#"Summary    : My summary
Description: My, description
Location   : https://www.example.com/
Organizer  : mailto:first.last@example.com
Dtstart    : Tue, 19 Dec 2023 11:00:00 +0100 (Tue, 19 Dec 2023 14:00:00 +0400)
Dtend      : Tue, 19 Dec 2023 12:00:00 +0100 (Tue, 19 Dec 2023 15:00:00 +0400)
"#
    );
}

#[test]
fn test_same_tz() {
    let mut ctx = TestContext::new(&["src/fixtures/same-tz.ics"]);

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        r#"Summary    : My summary
Description: My, description
Location   : https://www.example.com/
Organizer  : mailto:first.last@example.com
Dtstart    : Tue, 19 Dec 2023 14:00:00 +0100
Dtend      : Tue, 19 Dec 2023 15:00:00 +0100
"#
    );
}

/// The specified time is not valid with the given timezone (OffsetResult::None).
#[test]
fn test_none_tz() {
    let mut ctx = TestContext::new(&["src/fixtures/none-tz.ics"]);

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.time), 1);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        r#"Summary    : My summary
Description: My, description
Location   : https://www.example.com/
Organizer  : mailto:first.last@example.com
assume_timezone() failed
"#
    );
}

#[test]
fn test_missing_dtstart() {
    let mut ctx = TestContext::new(&["src/fixtures/missing-dtstart.ics"]);

    assert_eq!(main(ctx.get_args(), &mut ctx.buf, &ctx.time), 0);

    let buf = ctx.into_buf_string();
    assert_eq!(
        buf,
        r#"Summary    : My summary
Description: My, description
Location   : https://www.example.com/
Organizer  : mailto:first.last@example.com
Dtend      : Tue, 19 Dec 2023 15:00:00 +0100
"#
    );
}
