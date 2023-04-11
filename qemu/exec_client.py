#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

"""
This can run outside, on a host Linux machine:
- if you symlink the script script to e.g. ~/bin/winword, then it will inject the exe for you
- it'll try to map between the host and guest paths

Config file: ~/.config/qemu-exec-clientrc

[qemu-exec-client]
guest-ip = 192.168.x.y

You may want to customize:
- shared directory
- network drive letter
- guest port
"""

import configparser
import json
import os
import sys
import urllib.request


ALIASES = {
    "acroread": "c:/program files/adobe/acrobat dc/acrobat/acrobat.exe",
    "excel": "c:/program files/microsoft office/root/office16/excel.exe",
    "powerpnt": "c:/program files/microsoft office/root/office16/powerpnt.exe",
    "winword": "c:/program files/microsoft office/root/office16/winword.exe",
}


def get_config() -> configparser.ConfigParser:
    """Parses the config."""
    home = os.path.expanduser('~')
    config_home = os.environ.get('XDG_CONFIG_HOME') or os.path.join(home, '.config')
    config_path = os.path.join(config_home, 'qemu-exec-clientrc')
    config = configparser.ConfigParser()
    config.read(config_path)
    return config


def main() -> None:
    """Commandline interface to this module."""
    config = get_config()
    guest_ip = config.get('qemu-exec-client', 'guest-ip').strip()
    argv = sys.argv[1:]

    # If my name is an alias, inject it.
    my_name = sys.argv[0]
    command = ""
    for key, value in ALIASES.items():
        if not my_name.endswith(key):
            continue
        command = value
    if command:
        argv = [command] + argv

    # Map from host path to guest path.
    abs_argv = []
    for arg in argv:
        if os.path.exists(arg):
            if os.path.isabs(arg):
                abs_argv.append(arg)
            else:
                abs_argv.append(os.path.abspath(arg))
        else:
            abs_argv.append(arg)
    abs_argv = [i.replace(os.path.join(os.environ["HOME"], "git"), 'z:') for i in abs_argv]
    abs_argv = [i.replace("/", "\\") for i in abs_argv]

    payload_dict = {
        "command": abs_argv
    }
    payload = json.dumps(payload_dict)
    url = "http://{}:8000/exec".format(guest_ip)
    with urllib.request.urlopen(url, bytes(payload, "utf-8")) as stream:
        buf = stream.read()
        print(str(buf, "utf-8"))


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
