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
use pdfium_render::prelude::PdfDocument;
use pdfium_render::prelude::PdfMatrix;
use pdfium_render::prelude::PdfPage;
use pdfium_render::prelude::PdfPageImageObject;
use pdfium_render::prelude::PdfPageObjectCommon as _;
use pdfium_render::prelude::PdfPageObjectsCommon as _;
use pdfium_render::prelude::PdfPagePaperSize;
use pdfium_render::prelude::PdfPoints;
use pdfium_render::prelude::Pdfium;
use std::ffi::OsStr;
use std::io::Write as _;

/// Invokes the given external program with the given arguments, failing on a non-zero exit code.
fn run(debug: bool, program: &str, args: &[&OsStr]) -> anyhow::Result<()> {
    if debug {
        let args_joined = args
            .iter()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        print!("{program} {args_joined}...");
        std::io::stdout().flush()?;
    }
    let exit_status = std::process::Command::new(program).args(args).status()?;
    let exit_code = exit_status.code().context("code() failed")?;
    if exit_code != 0 {
        return Err(anyhow::anyhow!("{program} failed"));
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
            .value_parser(clap::value_parser!(u16).range(1..=12))
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

fn make_month_calendar<'a>(
    args: &Arguments,
    pdfium: &'a Pdfium,
    document: &mut PdfDocument<'a>,
    page: &mut PdfPage<'a>,
    matrix: PdfMatrix,
    month: &str,
) -> anyhow::Result<()> {
    let now = time::OffsetDateTime::now_utc();
    let next_year = (now.year() + 1).to_string();
    let locale = sys_locale::get_locales()
        .find(|i| !i.starts_with("C") && !i.starts_with("POSIX"))
        .context("no locale")?;
    let lang = locale
        .split(['-', '_'])
        .next()
        .context("split() failed")?
        .to_lowercase();
    let cal_ps = tempfile::Builder::new().suffix(".ps").tempfile()?;
    let config = format!("calendar_{lang}.txt");
    run(
        args.debug,
        "pcal",
        &[
            OsStr::new("-o"),
            cal_ps.path().as_os_str(),
            OsStr::new("-f"),
            OsStr::new(&config),
            OsStr::new(month),
            OsStr::new(&next_year),
        ],
    )?;
    let cal_pdf = tempfile::Builder::new().suffix(".pdf").tempfile()?;
    run(
        args.debug,
        "ps2pdf",
        &[cal_ps.path().as_os_str(), cal_pdf.path().as_os_str()],
    )?;
    let cal_doc = pdfium.load_pdf_from_file(cal_pdf.path(), None)?;
    let cal_pages = cal_doc.pages();
    let mut cal_page = cal_pages.get(0)?;
    let mut cal_object = cal_page
        .objects_mut()
        .copy_into_x_object_form_object(document)?;
    cal_object.move_to_page(page)?;
    cal_object.apply_matrix(matrix)?;

    Ok(())
}

fn make_month_image<'a>(
    args: &Arguments,
    document: &PdfDocument<'a>,
    page: &mut PdfPage<'a>,
    matrix: PdfMatrix,
    month: &str,
) -> anyhow::Result<()> {
    if args.debug {
        print!("making the image part...");
        std::io::stdout().flush()?;
    }
    let image_path = format!("images/{month}.jpg");
    // The JPEG is added to the PDF as-is, pdfium only wraps it in a stream
    // object, so it is not decoded and re-compressed at all.
    let mut image_object = PdfPageImageObject::new_from_jpeg_file(document, &image_path)
        .context(format!("failed to load {image_path}"))?;
    let landscape_size = PdfPagePaperSize::a4().landscape();
    // Larger top margin for the binding, no bottom margin since the calendar has one already.
    let margin_side = PdfPoints::from_mm(20.0);
    let margin_top = PdfPoints::from_mm(30.0);
    let image_bb_width = landscape_size.width() - margin_side * 2.0;
    let image_bb_height = landscape_size.height() - margin_top;
    // The size of the object is 1x1 point before it gets scaled, the image
    // itself is not affected by that.
    let pixel_ratio = image_object.width()? as f32 / image_object.height()? as f32;
    let image_width = PdfPoints::new(
        image_bb_width
            .value
            .min(image_bb_height.value * pixel_ratio),
    );
    let image_height = image_width / pixel_ratio;
    image_object.scale(image_width.value, image_height.value)?;
    image_object.translate(
        margin_side + (image_bb_width - image_width) / 2.0,
        PdfPoints::new(0.0),
    )?;
    let mut image_object = page.objects_mut().add_image_object(image_object)?;
    image_object.apply_matrix(matrix)?;
    if args.debug {
        println!("done");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Arguments::parse(&argv)?;
    let pdfium = Pdfium::default();

    let a4_size = PdfPagePaperSize::a4();
    let mut output_pdf = pdfium.create_new_pdf()?;
    let mut page = output_pdf.pages_mut().create_page_at_end(a4_size)?;
    create_grid(&args, &mut page)?;

    for month in 1..13 {
        println!("{month}...");
        let month_string = format!("{month:02}");

        // Portrait A4 page: upper half contains first calendar and the first image,
        // lower half contains the second calendar and the second image.
        let odd = month % 2 == 1;
        if odd && month > 1 {
            page = output_pdf.pages_mut().create_page_at_end(a4_size)?;
            create_grid(&args, &mut page)?;
        }

        let offset_y = if odd {
            a4_size.height()
        } else {
            a4_size.height() / 2.0
        };
        let calendar_matrix = PdfMatrix::IDENTITY
            .rotate_clockwise_degrees(90.0)?
            .scale(0.5, 0.5)?
            .translate(PdfPoints::new(0.0), offset_y)?;
        let image_matrix = calendar_matrix.translate(a4_size.width() / 2.0, PdfPoints::new(0.0))?;

        // Handle the calendar part.
        make_month_calendar(
            &args,
            &pdfium,
            &mut output_pdf,
            &mut page,
            calendar_matrix,
            &month_string,
        )?;

        // Handle the image part.
        make_month_image(&args, &output_pdf, &mut page, image_matrix, &month_string)?;

        if let Some(limit) = args.limit
            && month == limit
        {
            break;
        }
    }

    Ok(output_pdf.save_to_file("out.pdf")?)
}
