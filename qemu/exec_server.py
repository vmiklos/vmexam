#!/usr/bin/env python3
#
# Copyright 2021 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

"""
This can potentially auto-start inside a Windows VM:
- go to C:/Users/you/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup
- create an exec-server.lnk file in Explorer pointing to e.g.
  "c:/Python39/pythonw.exe c:/path/to/exec_server.py"

You may want to customize the port number below.

Needless to say, run this only in a VM that is only visible to your local host, not to any outside
network, since it allows executing arbitrary commands, remotely, without auth.
"""

from typing import Any
from typing import Dict
from typing import Iterable
from typing import TYPE_CHECKING
import json
import os
import subprocess
import traceback
import wsgiref.simple_server

if TYPE_CHECKING:
    # pylint: disable=no-name-in-module,import-error
    from wsgiref.types import StartResponse


def application(
        environ: Dict[str, Any],
        start_response: 'StartResponse'
) -> Iterable[bytes]:
    """The entry point of this WSGI app."""
    out = ""
    try:
        body = "OK"
        content_length = int(environ['CONTENT_LENGTH'])
        payload = environ["wsgi.input"].read(content_length).decode("utf-8")
        payload_dict = json.loads(payload)
        command = payload_dict["command"]
        detach = True
        if "sync" in payload_dict:
            detach = False
        if os.name == "nt" and detach:
            flags = 0
            flags |= 0x00000008  # DETACHED_PROCESS
            flags |= 0x00000200  # CREATE_NEW_PROCESS_GROUP
            flags |= 0x08000000  # CREATE_NO_WINDOW
            # pylint: disable=consider-using-with
            subprocess.Popen(command,
                             close_fds=True,  # close stdin/stdout/stderr on child
                             creationflags=flags)
        else:
            sync_process = subprocess.run(command, stdout=subprocess.PIPE, check=True)
            out = sync_process.stdout.decode("utf-8")
    # pylint: disable=broad-except
    except Exception:
        body = "KO"
        traceback.print_exc()

    status = '200 OK'
    headers = [('Content-type', 'text/plain; charset=utf-8')]
    if out:
        body = body + "\n" + out.strip()
    body_bytes = body.encode('utf-8')
    start_response(status, headers)
    return [body_bytes]


def main() -> None:
    """Commandline interface to this module."""
    httpd = wsgiref.simple_server.make_server('', 8000, application)
    httpd.serve_forever()


if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
