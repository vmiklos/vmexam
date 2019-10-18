#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

# Quick&dirty script to turn an 'SW_DEBUG=1 ./soffice' layout dump (produced by pressing F12) into
# an SVG file, which visualizes the relations between the layout frames.

from xml.dom import minidom
import sys

FONT_SIZE = 12
RECTANGLE_STYLE = "stroke: black; fill: none;"


def get_by_name(node, child_name):
    return [i for i in node.childNodes if i.localName == child_name][0]


def twip_to_pt(fro):
    return fro / 20


def print_text(x, y, text):
    print('  <text x="{}pt" y="{}pt" font-size="{}pt" dominant-baseline="hanging">{}</text>'.format(x, y, FONT_SIZE, text))


def print_rect(identifier, left, top, width, height):
    print('  <rect id="{}" x="{}pt" y="{}pt" width="{}pt" height="{}pt" style="{}"/>'.format(identifier, left, top, width, height, RECTANGLE_STYLE))


def is_frame_name(frame):
    return frame in ("page", "header", "footer", "body", "txt", "fly", "notxt")


def handle_frame(frame):
    identifier = ""
    symbol = ""
    for k, v in list(frame.attributes.items()):
        if k == "id":
            identifier = v
        elif k == "symbol":
            symbol = v
    infos = get_by_name(frame, "infos")
    infos_bounds = get_by_name(infos, "bounds")
    left = 0
    top = 0
    width = 0
    height = 0
    for k, v in list(infos_bounds.attributes.items()):
        if k == "left":
            left = twip_to_pt(float(v))
        elif k == "top":
            top = twip_to_pt(float(v))
        elif k == "width":
            width = twip_to_pt(float(v))
        elif k == "height":
            height = twip_to_pt(float(v))

    print_rect(identifier, left, top, width, height)
    print_text(left, top, "{} {}".format(symbol, identifier))

    for child in [i for i in frame.childNodes if is_frame_name(i.localName)]:
        handle_frame(child)

    anchoreds = [i for i in frame.childNodes if i.localName == "anchored"]
    if anchoreds:
        anchored = anchoreds[0]
        for child in [i for i in anchored.childNodes if is_frame_name(i.localName)]:
            handle_frame(child)


def main():
    print('<?xml version="1.0"?>')
    layout = minidom.parse(sys.argv[1])
    root = get_by_name(layout, "root")
    identifier = ""
    symbol = ""
    for k, v in list(root.attributes.items()):
        if k == "id":
            identifier = v
        elif k == "symbol":
            symbol = v

    root_infos = get_by_name(root, "infos")
    root_infos_bounds = get_by_name(root_infos, "bounds")
    left = 0
    top = 0
    width = 0
    height = 0
    for k, v in list(root_infos_bounds.attributes.items()):
        if k == "left":
            left = twip_to_pt(float(v))
        elif k == "top":
            top = twip_to_pt(float(v))
        elif k == "width":
            width = twip_to_pt(float(v))
        elif k == "height":
            height = twip_to_pt(float(v))
    # Root frame is the bounding box of all pages, canvas size is the same + the margins.
    print('<svg width="{}pt" height="{}pt" xmlns="http://www.w3.org/2000/svg">'.format(width + 2 * left, height + 2 * top))
    print_rect(identifier, left, top, width, height)
    print_text(left, top, "{} {}".format(symbol, identifier))

    for page in [i for i in root.childNodes if is_frame_name(i.localName)]:
        handle_frame(page)

    print('</svg>')


if __name__ == '__main__':
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
