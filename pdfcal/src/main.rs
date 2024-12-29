/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pdfcal.

use anyhow::Context as _;
use pdfium_render::prelude::PdfColor;
use pdfium_render::prelude::PdfPageImageObject;
use pdfium_render::prelude::PdfPageObjectsCommon as _;
use pdfium_render::prelude::PdfPagePaperSize;
use pdfium_render::prelude::PdfPoints;
use pdfium_render::prelude::Pdfium;

/// Converts a tempfile to a path that external commands can access.
fn tempfile_to_path(tempfile: &tempfile::NamedTempFile) -> anyhow::Result<String> {
    Ok(tempfile
        .path()
        .to_str()
        .context("to_str() failed")?
        .to_string())
}

/// Invokes 'pcal' with given arguments.
fn pcal(args: &[String]) -> anyhow::Result<()> {
    println!("pcal {}", args.join(" "));
    let exit_status = std::process::Command::new("pcal").args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("pcal failed"));
    }

    Ok(())
}

/// Invokes 'ps2pdf' with given arguments.
fn ps2pdf(ps: &str, pdf: &str) -> anyhow::Result<()> {
    let exit_status = std::process::Command::new("ps2pdf")
        .args(&[ps, pdf])
        .status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("ps2pdf failed"));
    }

    Ok(())
}

struct Arguments {
    debug: bool,
    limit: Option<u16>,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let debug_arg = clap::Arg::new("debug")
            .short('d')
            .long("debug")
            .action(clap::ArgAction::SetTrue)
            .help("Add debug output to the PDF, disabled by default");
        let limit_arg = clap::Arg::new("limit")
            .short('l')
            .long("limit")
            .value_parser(clap::value_parser!(u16))
            .required(false)
            .help("Limit the output to the first <limit> months, disabled by default");
        let args = [debug_arg, limit_arg];
        let app = clap::Command::new("pdfcal");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let debug = *matches.get_one::<bool>("debug").context("no debug arg")?;
        let limit = matches.get_one::<u16>("limit").cloned();
        Ok(Arguments { debug, limit })
    }
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;
    let pdfium = Pdfium::default();

    // A4: 210 x 297 mm.
    let a4_width = 595.275590551;
    let a4_height = 841.88976378;
    let a4_ratio = a4_height / a4_width;
    let mut output_pdf = pdfium.create_new_pdf()?;
    let mut page = output_pdf
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;
    if args.debug {
        page.objects_mut().create_path_object_line(
            PdfPoints::new(a4_width / 2_f32),
            PdfPoints::new(0_f32),
            PdfPoints::new(a4_width / 2_f32),
            PdfPoints::new(a4_height),
            PdfColor::new(255, 0, 0, 255),
            PdfPoints::new(3_f32),
        )?;
        page.objects_mut().create_path_object_line(
            PdfPoints::new(0.0),
            PdfPoints::new(a4_height / 2.0),
            PdfPoints::new(a4_width),
            PdfPoints::new(a4_height / 2.0),
            PdfColor::new(255, 0, 0, 255),
            PdfPoints::new(3.0),
        )?;
    }

    for month in 1..13 {
        println!("{month}...");
        let month_string = format!("{month:02}");

        // Handle the image part.
        let image = image::ImageReader::open(format!("images/{month_string}.jpg"))?.decode()?;
        // About 15 mm.
        let margin = 42.52;
        let image_bb_width = a4_width / 2_f32 - 2_f32 * margin;
        let image_bb_height = a4_height / 2_f32 - 2_f32 * margin;
        let pixel_ratio = image.width() as f32 / image.height() as f32;
        let image_width;
        let image_height;
        // Relative offset, inside the image bounding box.
        let image_offset_x;
        let image_offset_y;
        if a4_ratio < pixel_ratio {
            image_width = image_bb_height / pixel_ratio;
            image_height = image_bb_height;
            image_offset_x = (image_bb_width - image_width) / 2_f32 + margin;
            image_offset_y = -margin;
        } else {
            image_width = image_bb_width;
            image_height = image_bb_width * pixel_ratio;
            image_offset_x = margin;
            image_offset_y = -(image_bb_height - image_height) / 2_f32 - margin;
        }
        let page_image_object = PdfPageImageObject::new(&output_pdf, &image)?;
        let mut image_object = page.objects_mut().add_image_object(page_image_object)?;

        // Handle the calendar part.
        let now = time::OffsetDateTime::now_utc();
        let next_year = (now.year() + 1).to_string();
        let locale = sys_locale::get_locales()
            .filter(|i| i != "C")
            .next()
            .context("no locale")?;
        let lang = locale.split('-').next().context("split() failed")?;
        let cal_ps = tempfile::Builder::new().suffix(".ps").tempfile()?;
        let cal_ps_path = tempfile_to_path(&cal_ps)?;
        pcal(&[
            "-o".to_string(),
            cal_ps_path.to_string(),
            "-f".to_string(),
            format!("calendar_{lang}.txt"),
            month_string,
            next_year,
        ])?;
        let cal_pdf = tempfile::Builder::new().suffix(".pdf").tempfile()?;
        let cal_pdf_path = tempfile_to_path(&cal_pdf)?;
        ps2pdf(&cal_ps_path, &cal_pdf_path)?;
        let cal_doc = pdfium.load_pdf_from_file(&cal_pdf_path, None)?;
        let mut cal_object = page
            .objects_mut()
            .create_x_object_form_object(&cal_doc, 0)?;

        // Portrait A4 page: upper half contains first calendar and the first image,
        // lower half contains the second calendar and the second image.
        if month % 2 == 1 {
            if month > 1 {
                page = output_pdf
                    .pages_mut()
                    .create_page_at_end(PdfPagePaperSize::a4())?;
            }
            image_object.rotate_clockwise_degrees(90_f32)?;
            image_object.scale(image_width, image_height)?;
            image_object.translate(
                PdfPoints::new(a4_width / 2_f32 + image_offset_x),
                PdfPoints::new(a4_height + image_offset_y),
            )?;
            cal_object.rotate_clockwise_degrees(90_f32)?;
            cal_object.scale(0.5, 0.5)?;
            cal_object.translate(PdfPoints::new(0.0), PdfPoints::new(a4_height))?;
        } else {
            image_object.rotate_clockwise_degrees(90_f32)?;
            image_object.scale(image_width, image_height)?;
            image_object.translate(
                PdfPoints::new(a4_width / 2_f32 + image_offset_x),
                PdfPoints::new(a4_height / 2_f32 + image_offset_y),
            )?;
            cal_object.rotate_clockwise_degrees(90_f32)?;
            cal_object.scale(0.5, 0.5)?;
            cal_object.translate(PdfPoints::new(0.0), PdfPoints::new(a4_height / 2.0))?;
            page.regenerate_content()?;
        }

        if let Some(limit) = args.limit {
            if month == limit {
                break;
            }
        }
    }

    Ok(output_pdf.save_to_file("out.pdf")?)
}
