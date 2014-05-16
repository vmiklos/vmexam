#!/usr/bin/env python
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import supybot.conf as conf


def configure(advanced):
    conf.registerPlugin('Otrs', True)

Otrs = conf.registerPlugin('Otrs')

# vim:set shiftwidth=4 softtabstop=4 expandtab:
