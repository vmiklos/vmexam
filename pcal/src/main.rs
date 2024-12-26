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
        let image_height = a4_height / 2_f32;
        let image_width = a4_width / 2_f32;
        let image = image::ImageReader::open(format!("images/{month_string}.jpg"))?.decode()?;
        let mut image_object = PdfPageImageObject::new(&output_pdf, &image)?;
        image_object.rotate_clockwise_degrees(90_f32)?;
        // TODO preserve aspect ratio
        image_object.scale(image_width, image_height)?;
        // TODO margin
        image_object.translate(PdfPoints::new(a4_width / 2_f32), PdfPoints::new(a4_height))?;

        // Handle the calendar part.
        // TODO

        // Portrait A4 page: upper half contains first calendar and the first image,
        // lower half contains the second calendar and the second image.
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
