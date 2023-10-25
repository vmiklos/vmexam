/*
 * Copyright 2021 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

//! Dumps a Master Boot Record.
//!
//! See <https://wiki.osdev.org/MBR_(x86)> for a spec.

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use byteorder::ReadBytesExt as _;
use clap::Parser as _;
use std::io::Read as _;

#[derive(clap::Parser)]
struct Arguments {
    input: String,
}

type ByteCursor = std::io::Cursor<Vec<u8>>;

/// The MBR Bootstrap (flat binary executable code).
struct BootstrapRecord<'a> {
    cursor: &'a mut ByteCursor,
}

impl<'a> BootstrapRecord<'a> {
    fn new(cursor: &'a mut ByteCursor) -> Self {
        BootstrapRecord { cursor }
    }

    fn dump(&mut self) -> anyhow::Result<()> {
        println!("<bootstrap-record>");
        let pos = self.cursor.position();
        let mut word_count = 0;
        let mut byte_array = Vec::new();
        let mut array_start_pos = pos;
        while self.cursor.position() < pos + 440 {
            byte_array.push(self.cursor.read_u16::<byteorder::LittleEndian>()?);
            word_count += 1;
            if word_count == 8 {
                let offset = format!("{:04X}", array_start_pos);
                let byte_string = byte_array
                    .iter()
                    .map(|i| format!("{:04X}", i))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!(r#"<chunk offset="{}" bytes="{}"/>"#, offset, byte_string);
                word_count = 0;
                byte_array.clear();
                array_start_pos = self.cursor.position();
            }
        }
        if word_count != 0 {
            let offset = format!("{:04X}", array_start_pos);
            let byte_string = byte_array
                .iter()
                .map(|i| format!("{:04X}", i))
                .collect::<Vec<_>>()
                .join(" ");
            println!(r#"<chunk offset="{}" bytes="{}"/>"#, offset, byte_string);
        }
        println!("</bootstrap-record>");
        Ok(())
    }
}

/// Dumps a partition table entry.
struct PartitionRecord<'a> {
    cursor: &'a mut ByteCursor,
}

impl<'a> PartitionRecord<'a> {
    fn new(cursor: &'a mut ByteCursor) -> Self {
        PartitionRecord { cursor }
    }

    fn dump(&mut self) -> anyhow::Result<()> {
        println!("<partition-record>");
        println!(
            r#"<drive_attributes value="{:#x}"/>"#,
            self.cursor.read_u8()?
        );
        // See <https://wiki.osdev.org/Partition_Table>.
        println!(r#"<starting_chs_1 value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<starting_chs_2 value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<starting_chs_3 value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<partition_type value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<ending_chs_1 value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<ending_chs_2 value="{:#x}"/>"#, self.cursor.read_u8()?);
        println!(r#"<ending_chs_3 value="{:#x}"/>"#, self.cursor.read_u8()?);
        // start sector, 1 sector = 512 bytes
        println!(
            r#"<lba value="{:#x}"/>"#,
            self.cursor.read_u32::<byteorder::LittleEndian>()?
        );
        println!(
            r#"<sector_count value="{:#x}"/>"#,
            self.cursor.read_u32::<byteorder::LittleEndian>()?
        );
        println!("</partition-record>");
        Ok(())
    }
}

/// Toplevel record of an MBR byte array.
struct MbrStream<'a> {
    cursor: &'a mut ByteCursor,
}

impl<'a> MbrStream<'a> {
    fn new(cursor: &'a mut ByteCursor) -> Self {
        MbrStream { cursor }
    }

    fn dump(&mut self) -> anyhow::Result<()> {
        let pos = self.cursor.position();
        println!(
            r#"<stream type="MBR" size="{}">"#,
            self.cursor.get_ref().len()
        );
        BootstrapRecord::new(self.cursor).dump()?;
        println!(
            r#"<disk_id value="{:#x}"/>"#,
            self.cursor.read_u32::<byteorder::LittleEndian>()?
        );
        println!(
            r#"<reserved value="{:#x}"/>"#,
            self.cursor.read_u16::<byteorder::LittleEndian>()?
        );
        for _ in 0..4 {
            PartitionRecord::new(self.cursor).dump()?;
        }
        println!(
            r#"<signature value="{:#x}"/>"#,
            self.cursor.read_u16::<byteorder::LittleEndian>()?
        );
        println!("</stream>");
        assert_eq!(self.cursor.position(), pos + 512);
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let mut stream = std::fs::File::open(args.input)?;
    let mut data = vec![];
    stream.read_to_end(&mut data)?;
    let mut cursor = std::io::Cursor::new(data);
    let mut mbr_stream = MbrStream::new(&mut cursor);
    println!(r#"<?xml version="1.0"?>"#);
    mbr_stream.dump()?;
    Ok(())
}
