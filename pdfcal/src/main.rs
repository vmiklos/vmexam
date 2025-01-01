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
use pdfium_render::prelude::PdfPage;
use pdfium_render::prelude::PdfPageObjectsCommon as _;
use pdfium_render::prelude::PdfPagePaperSize;
use pdfium_render::prelude::PdfPoints;
use pdfium_render::prelude::Pdfium;
use std::io::Write as _;

/// Converts a tempfile to a path that external commands can access.
fn tempfile_to_path(tempfile: &tempfile::NamedTempFile) -> anyhow::Result<String> {
    Ok(tempfile
        .path()
        .to_str()
        .context("to_str() failed")?
        .to_string())
}

/// Invokes 'pcal' with given arguments.
fn pcal(debug: bool, args: &[String]) -> anyhow::Result<()> {
    if debug {
        print!("pcal {}...", args.join(" "));
        std::io::stdout().flush()?;
    }
    let exit_status = std::process::Command::new("pcal").args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("pcal failed"));
    }
    if debug {
        println!("done");
    }

    Ok(())
}

/// Invokes 'ps2pdf' with given arguments.
fn ps2pdf(debug: bool, ps: &str, pdf: &str) -> anyhow::Result<()> {
    let args = [ps, pdf];
    if debug {
        print!("ps2pdf {}...", args.join(" "));
        std::io::stdout().flush()?;
    }
    let exit_status = std::process::Command::new("ps2pdf").args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("ps2pdf failed"));
    }
    if debug {
        println!("done");
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

fn create_grid(args: &Arguments, page: &mut PdfPage) -> anyhow::Result<()> {
    if args.debug {
        let a4_size = PdfPagePaperSize::a4();
        page.objects_mut().create_path_object_line(
            a4_size.width() / 2.0,
            PdfPoints::new(0.0),
            a4_size.width() / 2.0,
            a4_size.height(),
            PdfColor::new(255, 0, 0, 255),
            PdfPoints::new(3.0),
        )?;
        page.objects_mut().create_path_object_line(
            PdfPoints::new(0.0),
            a4_size.height() / 2.0,
            a4_size.width(),
            a4_size.height() / 2.0,
            PdfColor::new(255, 0, 0, 255),
            PdfPoints::new(3.0),
        )?;
    }

    Ok(())
}

fn make_month_calendar(
    args: &Arguments,
    pdfium: &Pdfium,
    page: &mut PdfPage,
    odd: bool,
    month: &str,
) -> anyhow::Result<()> {
    let now = time::OffsetDateTime::now_utc();
    let next_year = (now.year() + 1).to_string();
    let locale = sys_locale::get_locales()
        .find(|i| i != "C")
        .context("no locale")?;
    let lang = locale.split('-').next().context("split() failed")?;
    let cal_ps = tempfile::Builder::new().suffix(".ps").tempfile()?;
    let cal_ps_path = tempfile_to_path(&cal_ps)?;
    pcal(
        args.debug,
        &[
            "-o".to_string(),
            cal_ps_path.to_string(),
            "-f".to_string(),
            format!("calendar_{lang}.txt"),
            month.to_string(),
            next_year,
        ],
    )?;
    let cal_pdf = tempfile::Builder::new().suffix(".pdf").tempfile()?;
    let cal_pdf_path = tempfile_to_path(&cal_pdf)?;
    ps2pdf(args.debug, &cal_ps_path, &cal_pdf_path)?;
    let cal_doc = pdfium.load_pdf_from_file(&cal_pdf_path, None)?;
    let mut cal_object = page
        .objects_mut()
        .create_x_object_form_object(&cal_doc, 0)?;

    let a4_size = PdfPagePaperSize::a4();
    cal_object.rotate_clockwise_degrees(90.0)?;
    cal_object.scale(0.5, 0.5)?;
    if odd {
        cal_object.translate(PdfPoints::new(0.0), a4_size.height())?;
    } else {
        cal_object.translate(PdfPoints::new(0.0), a4_size.height() / 2.0)?;
    }

    Ok(())
}

fn make_month_image(
    args: &Arguments,
    page: &mut PdfPage,
    odd: bool,
    month: &str,
) -> anyhow::Result<()> {
    if args.debug {
        print!("making the image part...");
        std::io::stdout().flush()?;
    }
    let image = image::ImageReader::open(format!("images/{month}.jpg"))?.decode()?;
    // Top/right/bottom margin, no left (that is provided by pcal).
    let margin = PdfPoints::from_mm(15.0);
    let a4_size = PdfPagePaperSize::a4();
    let a4_ratio = a4_size.height().value / a4_size.width().value;
    let image_bb_width = a4_size.width() / 2.0 - margin;
    let image_bb_height = a4_size.height() / 2.0 - margin * 2.0;
    let pixel_ratio = image.width() as f32 / image.height() as f32;
    let image_width;
    let image_height;
    // Relative offset, inside the image bounding box.
    let image_offset_x;
    let image_offset_y;
    if a4_ratio < pixel_ratio {
        image_width = image_bb_height / pixel_ratio;
        image_height = image_bb_height;
        image_offset_x = (image_bb_width - image_width) / 2.0;
        image_offset_y = -margin;
    } else {
        image_width = image_bb_width;
        image_height = image_bb_width * pixel_ratio;
        image_offset_x = PdfPoints::new(0.0);
        image_offset_y = -(image_bb_height - image_height) / 2.0 - margin;
    }
    let mut image_object = page.objects_mut().create_image_object(
        PdfPoints::new(0.0),
        PdfPoints::new(0.0),
        &image,
        None,
        None,
    )?;
    image_object.rotate_clockwise_degrees(90.0)?;
    image_object.scale(image_width.value, image_height.value)?;
    if odd {
        image_object.translate(
            a4_size.width() / 2.0 + image_offset_x,
            a4_size.height() + image_offset_y,
        )?;
    } else {
        image_object.translate(
            a4_size.width() / 2.0 + image_offset_x,
            a4_size.height() / 2.0 + image_offset_y,
        )?;
    }
    if args.debug {
        println!("done");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;
    let pdfium = Pdfium::default();

    let mut output_pdf = pdfium.create_new_pdf()?;
    let mut page = output_pdf
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;
    create_grid(&args, &mut page)?;

    for month in 1..13 {
        println!("{month}...");
        let month_string = format!("{month:02}");

        // Portrait A4 page: upper half contains first calendar and the first image,
        // lower half contains the second calendar and the second image.
        let odd = month % 2 == 1;
        if odd && month > 1 {
            page = output_pdf
                .pages_mut()
                .create_page_at_end(PdfPagePaperSize::a4())?;
        }

        // Handle the calendar part.
        make_month_calendar(&args, &pdfium, &mut page, odd, &month_string)?;

        // Handle the image part.
        make_month_image(&args, &mut page, odd, &month_string)?;

        if !odd {
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
