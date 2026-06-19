#!/usr/bin/env python3
#
# Copyright 2026 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#
# Provides /r to reply to a previous message, works with both Matrix and Mattermost.

import weechat

SCRIPT_NAME = "reply"
SCRIPT_AUTHOR = "matrirc"
SCRIPT_VERSION = "0.1.0"
SCRIPT_LICENSE = "GPL3"
SCRIPT_DESC = "Reply to a message ID in Mattermost or Matrix buffers"

def reply_cmd_cb(data, buffer, args):
    server_name = weechat.buffer_get_string(buffer, "localvar_server")

    if server_name and server_name.endswith("mm"):
        parts = args.strip().split(None, 1)
        if not parts:
            weechat.prnt(buffer, f"{weechat.prefix('error')}Error: Message ID is required.")
            return weechat.WEECHAT_RC_ERROR
        msg_id = parts[0]
        rest = parts[1] if len(parts) > 1 else ""
        msg = f"@@{msg_id} {rest}" if rest else f"@@{msg_id}"
        weechat.command(buffer, msg)
    elif server_name and server_name.startswith("matrix-"):
        parts = args.strip().split(None, 1)
        if not parts:
            weechat.prnt(buffer, f"{weechat.prefix('error')}Error: Message ID is required.")
            return weechat.WEECHAT_RC_ERROR
        msg_id = parts[0]
        rest = parts[1] if len(parts) > 1 else ""
        msg = f"!r {msg_id} {rest}" if rest else f"!r {msg_id}"
        weechat.command(buffer, msg)
    else:
        weechat.prnt(buffer, f"{weechat.prefix('error')}Error: The server was not recognized as Mattermost or Matrix.")

    return weechat.WEECHAT_RC_OK

if weechat.register(SCRIPT_NAME, SCRIPT_AUTHOR, SCRIPT_VERSION, SCRIPT_LICENSE, SCRIPT_DESC, "", ""):
    weechat.hook_command(
        "r",
        "Reply to a message ID",
        "ID [message]",
        "ID: message identifier\nmessage: reply body",
        "",
        "reply_cmd_cb",
        ""
    )
