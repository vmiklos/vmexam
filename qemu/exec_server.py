#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

"""
This can potentially auto-start inside a Windows VM:
- go to C:/Users/you/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup
- create an exec-server.lnk file in Explorer pointing to e.g.
  "c:/Python39/pythonw.exe c:/lo/exec_server.py"

You may want to customize the port number below.

Needless to say, run this only in a VM that is only visible to your local host, not to any outside
network, since it allows executing arbitrary commands, remotely, without auth.
"""

import json
import os
import subprocess
import traceback
import wsgiref.simple_server


def application(environ, start_response):
    """The entry point of this WSGI app."""
    try:
        body = "OK"
        content_length = int(environ['CONTENT_LENGTH'])
        payload = environ["wsgi.input"].read(content_length).decode("utf-8")
        payload_dict = json.loads(payload)
        command = payload_dict["command"]
        if os.name == "nt":
            flags = 0
            flags |= 0x00000008  # DETACHED_PROCESS
            flags |= 0x00000200  # CREATE_NEW_PROCESS_GROUP
            flags |= 0x08000000  # CREATE_NO_WINDOW
            pkwargs = {
                'close_fds': True,  # close stdin/stdout/stderr on child
                'creationflags': flags,
            }
            subprocess.Popen(command, **pkwargs)
        else:
            subprocess.run(command, check=True)
    # pylint: disable=broad-except
    except Exception:
        body = "KO"
        traceback.print_exc()

    status = '200 OK'
    headers = [('Content-type', 'text/plain; charset=utf-8')]
    body_bytes = body.encode('utf-8')
    start_response(status, headers)
    return [body_bytes]


def main():
    """Commandline interface to this module."""
    httpd = wsgiref.simple_server.make_server('', 8000, application)
    httpd.serve_forever()


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
