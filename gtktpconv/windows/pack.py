#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import os
import shutil
import zipfile


def main():
    instdir = "gtktpconv"
    if os.path.exists(instdir):
        shutil.rmtree(instdir)
    os.mkdir(instdir)
    shutil.copyfile("workdir/Release/gtktpconv.exe", instdir + "/gtktpconv.exe")
    dlls = [
        "gtk-3-3.0.dll",
        "gdk-3-3.0.dll",
        "atk-1.0.dll",
        "glib-2.0.dll",
        "gio-2.0.dll",
        "gmodule-2.0.dll",
        "gobject-2.0.dll",
        "intl.dll",
        "iconv.dll",
        "pangowin32-1.0.dll",
        "pango-1.0.dll",
        "pangocairo-1.0.dll",
        "cairo.dll",
        "libpng16.dll",
        "zlib1.dll",
        "fontconfig.dll",
        "libxml2.dll",
        "pangoft2-1.0.dll",
        "harfbuzz.dll",
        "cairo-gobject.dll",
        "gdk_pixbuf-2.0.dll",
        "epoxy-0.dll",
    ]
    for dll in dlls:
        shutil.copyfile("c:/gtk-build/gtk/Win32/release/bin/" + dll, instdir + "/" + dll)
    sysdlls = [
        "msvcr120.dll",
        "msvcp120.dll",
    ]
    for dll in sysdlls:
        shutil.copyfile("c:/windows/system32/" + dll, instdir + "/" + dll)

    with zipfile.ZipFile(instdir + ".zip", "w") as zip:
        for root, dirs, files in os.walk("gtktpconv"):
            for name in files:
                if not name == "gtktpconv":
                    zip.write(os.path.join(root, name))

if __name__ == '__main__':
    main()

# vim:set filetype=python shiftwidth=4 softtabstop=4 expandtab:
