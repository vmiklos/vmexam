/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pdfcal.

use pdfium_render::prelude::PdfPage;
use pdfium_render::prelude::PdfPageImageObject;
use pdfium_render::prelude::PdfPageObjectsCommon as _;
use pdfium_render::prelude::PdfPagePaperSize;
use pdfium_render::prelude::PdfPoints;
use pdfium_render::prelude::Pdfium;

fn main() -> anyhow::Result<()> {
    let pdfium = Pdfium::default();

    // A4: 210 x 297 mm.
    let a4_width = 595.275590551;
    let a4_height = 841.88976378;
    let mut output_pdf = pdfium.create_new_pdf()?;
    let mut page: PdfPage;

    for month in 1..13 {
        println!("{month}...");
        let month_string = format!("{month:02}");

        // Handle the image part.
        // TOP_OF_CAL_BOXES_PTS in pcal's pcaldefs.h; 50% more so to have enough space for the
        // spiraling.
        let margin = 85_f32 * 1.5;
        let image_height = PdfPoints::new(a4_height - margin);
        let image_width = PdfPoints::new(a4_width - margin);
        let image = image::ImageReader::open(format!("images/{month_string}.jpg"))?.decode()?;
        let image_object =
            PdfPageImageObject::new_with_size(&output_pdf, &image, image_width, image_height)?;
        // TODO
        // .translate()
        // .rotate_clockwise_degrees()

        // Handle the calendar part.
        // TODO

        // Portrait A4 page: upper half contains first calendar and the first image,
        // lower half contains the second calendar and the second image.
        //let scale = 1_f32 / 2_f32;
        if month % 2 == 1 {
            page = output_pdf
                .pages_mut()
                .create_page_at_end(PdfPagePaperSize::a4())?;
            page.objects_mut().add_image_object(image_object)?;
        } else {
            // TODO
            break;
        }
    }

    Ok(output_pdf.save_to_file("out.pdf")?)
}
