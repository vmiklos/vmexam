/*
 * Copyright 2024 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to pdfcal.

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
    let a4_ratio = a4_height / a4_width;
    let mut output_pdf = pdfium.create_new_pdf()?;
    let mut page = output_pdf
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

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
        let mut image_object = PdfPageImageObject::new(&output_pdf, &image)?;

        // Handle the calendar part.
        // TODO

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
        } else {
            image_object.rotate_clockwise_degrees(90_f32)?;
            image_object.scale(image_width, image_height)?;
            image_object.translate(
                PdfPoints::new(a4_width / 2_f32 + image_offset_x),
                PdfPoints::new(a4_height / 2_f32 + image_offset_y),
            )?;
        }
        page.objects_mut().add_image_object(image_object)?;
        if month == 2 {
            break;
        }
    }

    Ok(output_pdf.save_to_file("out.pdf")?)
}
