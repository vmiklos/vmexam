#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#
# Parses clang-pp output and detects redundant preprocessor directives. Only
# #ifndef for now.
#

sock = open("out.yaml")

inEntry = False

stack = []
entry = {}

for line in sock.readlines():
    line = line.strip()
    if line.startswith("- Callback:"):
        if inEntry:
            if entry["Callback"] == "Ifndef":
                for s in stack:
                    if s["MacroNameTok"] == entry["MacroNameTok"]:
                        print("nested ifndef of '"+entry["MacroNameTok"]+"', current location is '"+entry["Loc"]+"', previous loc is '"+s["Loc"]+"'")
                stack.append(entry)
            elif entry["Callback"] == "Endif" and len(stack) and entry["IfLoc"] == stack[-1]["Loc"]:
                stack.pop()
            entry = {}
        inEntry = True
        entry["Callback"] = line[len("- Callback: "):]
    elif inEntry and line.startswith("Loc: "):
        entry["Loc"] = line[len("Loc: "):]
    elif inEntry and line.startswith("IfLoc: "):
        entry["IfLoc"] = line[len("IfLoc: "):]
    elif inEntry and line.startswith("MacroNameTok: "):
        entry["MacroNameTok"] = line[len("MacroNameTok: "):]

sock.close()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
