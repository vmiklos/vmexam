#!/usr/bin/env python3
# -*- coding: UTF-8 -*-
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# pdfcal: builds on top of pcal, adding image support.

# TODO:
# - don't expect images in .pdf (A4 landscape) format under images/
# - don't expect month calendars in .pdf (A4 landscape) under images/
# - A5 output instead of A4 output

import PyPDF2
import math

# A4: 210 x 297 mm.
a4Width = 595.275590551
a4Height = 841.88976378

outputPdf = PyPDF2.PdfFileWriter()

for month in range(1, 13):
    monthString = "%02d" % month

    imagePdf = PyPDF2.PdfFileReader(open("images/img" + monthString + ".pdf", "rb"))
    imagePage = imagePdf.getPage(0)
    calPdf = PyPDF2.PdfFileReader(open("images/cal" + monthString + ".pdf", "rb"))
    calPage = calPdf.getPage(0)

    # Portrait A4 page: upper half contains the image, lower half contains the
    # month calendar.
    page = PyPDF2.pdf.PageObject.createBlankPage(outputPdf, width=a4Width, height=a4Height)
    sqrt2 = 1. / math.sqrt(2)
    page.mergeScaledTranslatedPage(imagePage, scale=sqrt2, tx=0, ty=a4Height / 2)
    page.mergeRotatedScaledTranslatedPage(calPage, rotation=-90, scale=sqrt2, tx=0, ty=a4Height / 2, expand=True)

    outputPdf.addPage(page)

outputPdf.write(open("out.pdf", "wb"))

# vim:set shiftwidth=4 softtabstop=4 expandtab:
