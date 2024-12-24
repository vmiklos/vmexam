#!/usr/bin/env python3
# -*- coding: UTF-8 -*-
#
# Copyright 2018 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

"""pdfcal: builds on top of pcal, adding image support."""

import io
import locale
import subprocess
import time

import PIL.ImageFile
import img2pdf
import pypdf


def ps2pdf(ps):
    """Converts ps as a buffer-like object containing PS, and converts it to PDF."""
    pdf = io.BytesIO()

    args = ["ps2pdf", "-", "-"]
    with subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE) as sock:
        sock.stdin.write(ps.read())
        sock.stdin.close()
        pdf.write(sock.stdout.read())
        sock.stdout.close()
    pdf.seek(0)

    return pdf


def pcal(args):
    """Invokes 'pcal' with given arguments and returns its output as a buffer-like object."""
    ps = io.BytesIO()

    with subprocess.Popen(["pcal"] + args, stdout=subprocess.PIPE) as sock:
        ps.write(sock.stdout.read())
        sock.stdout.close()
    ps.seek(0)

    return ps


def make_transform(rotate, scale, tx, ty):
    """Creates a transform with a specified rotation, scaling and offsets."""
    t = pypdf.Transformation()
    return t.rotate(rotate).scale(scale, scale).translate(tx, ty)


def make_pdf_page(image_buf):
    """Creates a PDF page from a byte array."""
    image_pdf = pypdf.PdfReader(image_buf)
    return image_pdf.pages[0]


def make_image_page(a4_height, a4_width, month):
    """Creates the image part of a month page."""
    # Landscape A4 for the image.
    page_size = (a4_height, a4_width)
    # TOP_OF_CAL_BOXES_PTS in pcal's pcaldefs.h; 50% more so to have enough space for the
    # spiraling.
    margin = 85 * 1.5
    image_height = (img2pdf.ImgSize.abs, a4_height - margin)
    image_width = (img2pdf.ImgSize.abs, a4_width - margin)
    image_size = (image_height, image_width)
    layout_fun = img2pdf.get_layout_fun(page_size, image_size)
    image_buf = io.BytesIO()
    with open("images/" + month + ".jpg", "rb") as image_jpg:
        img2pdf.convert(image_jpg, layout_fun=layout_fun, outputstream=image_buf)
    image_buf.seek(0)
    return make_pdf_page(image_buf)


def main():
    """Commandline interface."""
    # Don't refuse loading certain JPEG files if imagemagick is doing so.
    PIL.ImageFile.LOAD_TRUNCATED_IMAGES = True

    # A4: 210 x 297 mm.
    a4_width = 595.275590551
    a4_height = 841.88976378

    output_pdf = pypdf.PdfWriter()

    page = None
    for month in range(1, 13):
        print(f"{month}...")
        month_string = f"{month:02}"

        # Handle the image part.
        image_page = make_image_page(a4_height, a4_width, month_string)

        # Handle the calendar part.
        next_year = str(time.localtime().tm_year + 1)
        lang = locale.getlocale()[0].split("_")[0]
        ps = pcal(["-f", "calendar_" + lang + ".txt", month_string, next_year])
        cal_page = make_pdf_page(ps2pdf(ps))

        # Portrait A4 page: upper half contains first calendar and the first image,
        # lower half contains the second calendar and the second image.
        scale = 1. / 2
        if month % 2 == 1:
            page = pypdf.PageObject.create_blank_page(output_pdf, width=a4_width, height=a4_height)
            trans = make_transform(-90, scale, a4_width / 2, a4_height)
            page.merge_transformed_page(image_page, trans)
            trans = make_transform(180, scale, a4_width / 2, a4_height)
            page.merge_transformed_page(cal_page, trans)
        else:
            trans = make_transform(-90, scale, a4_width / 2, a4_height / 2)
            page.merge_transformed_page(image_page, trans)
            trans = make_transform(180, scale, a4_width / 2, a4_height / 2)
            page.merge_transformed_page(cal_page, trans)
            output_pdf.add_page(page)

    with open("out.pdf", "wb") as stream:
        output_pdf.write(stream)


if __name__ == '__main__':
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
