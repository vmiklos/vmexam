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
use pdfium_render::prelude::PdfPage;
use pdfium_render::prelude::PdfPageObjectCommon as _;
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
    a4: bool,
}

impl Arguments {
    fn parse(argv: &[String]) -> anyhow::Result<Self> {
        let debug_arg = clap::Arg::new("debug")
            .short('d')
            .long("debug")
            .action(clap::ArgAction::SetTrue)
            .help("Add debug output to the PDF, disabled by default");
        let a4_arg = clap::Arg::new("a4")
            .short('a')
            .long("a4")
            .action(clap::ArgAction::SetTrue)
            .help("Use A4 (and not A5) as size for one month, disabled by default");
        let limit_arg = clap::Arg::new("limit")
            .short('l')
            .long("limit")
            .value_parser(clap::value_parser!(u16))
            .required(false)
            .help("Limit the output to the first <limit> months, disabled by default");
        let args = [debug_arg, limit_arg, a4_arg];
        let app = clap::Command::new("pdfcal");
        let matches = app.args(&args).try_get_matches_from(argv)?;
        let debug = *matches.get_one::<bool>("debug").context("no debug arg")?;
        let limit = matches.get_one::<u16>("limit").cloned();
        let a4 = *matches.get_one::<bool>("a4").context("no a4 arg")?;
        Ok(Arguments { debug, limit, a4 })
    }
}

fn create_grid(args: &Arguments, page: &mut PdfPage) -> anyhow::Result<()> {
    if args.debug {
        let a4_size = PdfPagePaperSize::a4();
        if !args.a4 {
            page.objects_mut().create_path_object_line(
                a4_size.width() / 2.0,
                PdfPoints::new(0.0),
                a4_size.width() / 2.0,
                a4_size.height(),
                PdfColor::new(255, 0, 0, 255),
                PdfPoints::new(3.0),
            )?;
        }
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
    let mut cal_object = cal_doc
        .pages()
        .get(0)?
        .objects()
        .copy_into_x_object_form_object(document)?;
    cal_object.move_to_page(page)?;

    let a4_size = PdfPagePaperSize::a4();
    if args.a4 {
        let a4_to_a5 = a4_size.width().value / a4_size.height().value;
        cal_object.scale(a4_to_a5, a4_to_a5)?;
    } else {
        cal_object.rotate_clockwise_degrees(90.0)?;
        cal_object.scale(0.5, 0.5)?;
        if odd {
            cal_object.translate(PdfPoints::new(0.0), a4_size.height())?;
        } else {
            cal_object.translate(PdfPoints::new(0.0), a4_size.height() / 2.0)?;
        }
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
    let image_path = format!("images/{month}.jpg");
    let image = image::ImageReader::open(&image_path)
        .context(format!("failed to open {image_path}"))?
        .decode()?;
    // Top/right/bottom margin, no left (that is provided by pcal).
    // For a4, no bottom.
    let margin = PdfPoints::from_mm(15.0);
    let a4_size = PdfPagePaperSize::a4();
    let a4_ratio = a4_size.height().value / a4_size.width().value;
    let image_bb_width = if args.a4 {
        a4_size.width() - margin * 2.0
    } else {
        a4_size.width() / 2.0 - margin
    };
    let image_bb_height = if args.a4 {
        a4_size.height() / 2.0 - margin
    } else {
        a4_size.height() / 2.0 - margin * 2.0
    };
    let pixel_ratio = if args.a4 {
        image.height() as f32 / image.width() as f32
    } else {
        image.width() as f32 / image.height() as f32
    };
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
        image_offset_x = if args.a4 {
            (image_bb_width - image_width) / 2.0 + margin
        } else {
            PdfPoints::new(0.0)
        };
        image_offset_y = if args.a4 {
            (image_bb_height - image_height) / 2.0
        } else {
            -(image_bb_height - image_height) / 2.0 - margin
        };
    }
    let mut image_object = page.objects_mut().create_image_object(
        PdfPoints::new(0.0),
        PdfPoints::new(0.0),
        &image,
        None,
        None,
    )?;
    if !args.a4 {
        image_object.rotate_clockwise_degrees(90.0)?;
    }
    image_object.scale(image_width.value, image_height.value)?;
    if args.a4 {
        image_object.translate(image_offset_x, a4_size.height() / 2.0 + image_offset_y)?;
    } else if odd {
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
        let a4 = args.a4;
        if (odd || a4) && month > 1 {
            page = output_pdf
                .pages_mut()
                .create_page_at_end(PdfPagePaperSize::a4())?;
        }

        // Handle the calendar part.
        make_month_calendar(
            &args,
            &pdfium,
            &mut output_pdf,
            &mut page,
            odd,
            &month_string,
        )?;

        // Handle the image part.
        make_month_image(&args, &mut page, odd, &month_string)?;

        if !(odd || a4) {
            page.regenerate_content()?;
        }

        if let Some(limit) = args.limit
            && month == limit
        {
            break;
        }
    }

    Ok(output_pdf.save_to_file("out.pdf")?)
}
