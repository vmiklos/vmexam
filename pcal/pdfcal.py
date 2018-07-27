#!/usr/bin/env python3
# -*- coding: UTF-8 -*-
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# pdfcal: builds on top of pcal, adding image support.

import PIL.ImageFile
import PyPDF2
import img2pdf
import io
import locale
import subprocess
import time


# Converts inPs as a buffer-like object containing PS, and converts it to PDF.
def ps2Pdf(inPs):
    bufPdf = io.BytesIO()

    sock = subprocess.Popen(["ps2pdf", "-", "-"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    sock.stdin.write(inPs.read())
    sock.stdin.close()
    bufPdf.write(sock.stdout.read())
    sock.stdout.close()
    bufPdf.seek(0)

    return bufPdf


# Invokes 'pcal' with given arguments and returns its output as a buffer-like object.
def pcal(args):
    bufPs = io.BytesIO()

    sock = subprocess.Popen(["pcal"] + args, stdout=subprocess.PIPE)
    bufPs.write(sock.stdout.read())
    sock.stdout.close()
    bufPs.seek(0)

    return bufPs


# Don't refuse loading certain JPEG files if imagemagick is doing so.
PIL.ImageFile.LOAD_TRUNCATED_IMAGES = True

# A4: 210 x 297 mm.
a4Width = 595.275590551
a4Height = 841.88976378

outputPdf = PyPDF2.PdfFileWriter()

page = None
for month in range(1, 13):
    monthString = "%02d" % month

    # Handle the image part.
    imageJpg = open("images/" + monthString + ".jpg", "rb")
    # Landscape A4 for the image.
    pageSize = (a4Height, a4Width)
    # TOP_OF_CAL_BOXES_PTS in pcal's pcaldefs.h.
    margin = 85
    imageSize = ((img2pdf.ImgSize.abs, a4Height - margin), (img2pdf.ImgSize.abs, a4Width - margin))
    layoutFun = img2pdf.get_layout_fun(pageSize, imageSize, border=None, fit=None, auto_orient=False)
    imageBytes = img2pdf.convert(imageJpg, layout_fun=layoutFun)
    imageBuf = io.BytesIO()
    imageBuf.write(imageBytes)
    imageBuf.seek(0)

    # Handle the calendar part.
    imagePdf = PyPDF2.PdfFileReader(imageBuf)
    imagePage = imagePdf.getPage(0)
    nextYear = str(time.localtime().tm_year + 1)
    lang = locale.getlocale()[0].split("_")[0]
    calPdf = PyPDF2.PdfFileReader(ps2Pdf(pcal(["-f", "calendar_" + lang + ".txt", monthString, nextYear])))
    calPage = calPdf.getPage(0)

    # Portrait A4 page: upper half contains first calendar and the first image,
    # lower half contains the second calendar and the second image.
    scale = 1. / 2
    if month % 2 == 1:
        page = PyPDF2.pdf.PageObject.createBlankPage(outputPdf, width=a4Width, height=a4Height)
        page.mergeRotatedScaledTranslatedPage(imagePage, rotation=-90, scale=scale, tx=a4Width / 2, ty=a4Height)
        page.mergeRotatedScaledTranslatedPage(calPage, rotation=180, scale=scale, tx=a4Width / 2, ty=a4Height)
    else:
        page.mergeRotatedScaledTranslatedPage(imagePage, rotation=-90, scale=scale, tx=a4Width / 2, ty=a4Height / 2)
        page.mergeRotatedScaledTranslatedPage(calPage, rotation=180, scale=scale, tx=a4Width / 2, ty=a4Height / 2)
        outputPdf.addPage(page)

outputPdf.write(open("out.pdf", "wb"))
# This can be optimized further by running e.g. 'gs -dNOPAUSE -dBATCH -dSAFER
# -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -sOutputFile=smaller.pdf out.pdf'.

# vim:set shiftwidth=4 softtabstop=4 expandtab:
