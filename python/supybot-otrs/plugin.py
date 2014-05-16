#!/usr/bin/env python
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import re
import os
import supybot.utils as utils
from supybot.commands import *
import supybot.plugins as plugins
import supybot.ircutils as ircutils
import supybot.callbacks as callbacks


class Otrs(callbacks.Plugin):
    threaded = True

    def __init__(self, irc):
        self.__parent = super(Otrs, self)
        self.__parent.__init__(irc)

        import ConfigParser
        config = ConfigParser.RawConfigParser()
        config.read(os.path.expanduser('~/.otrsrc'))
        self.url = config.sections()[0]
        self.user = config.get(self.url, "soap_user")
        self.password = config.get(self.url, "soap_password")
        self.prefix = config.get(self.url, "prefix")

    def _ticket_id(self, number):
        #
        # Example ~/.otrsrc:
        #
        # [https://localhost/otrs/rpc.pl]
        # soap_user = mycompany
        # soap_password = secret
        # prefix = bug#
        #

        import os
        import base64
        import urllib2
        import sys
        from xml.dom import minidom

        headers = {}
        headers['Authorization'] = 'Basic ' + base64.encodestring('%s:%s' % (self.user, self.password)).strip()
        headers['SOAPAction'] = '"Core#Dispatch"'
        headers['Content-Type'] = 'text/xml; charset=utf-8'

        body = """<?xml version="1.0" encoding="UTF-8"?>
        <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="Core" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" SOAP-ENV:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
          <SOAP-ENV:Body>
            <ns1:Dispatch>
              <param0 xsi:type="xsd:string">%s</param0>
              <param1 xsi:type="xsd:string">%s</param1>
              <param2 xsi:type="xsd:string">TicketObject</param2>
              <param3 xsi:type="xsd:string">TicketIDLookup</param3>
              <param4 xsi:type="xsd:string">TicketNumber</param4>
              <param5 xsi:type="xsd:string">%s</param5>
            </ns1:Dispatch>
          </SOAP-ENV:Body>
        </SOAP-ENV:Envelope>""" % (self.user, self.password, number)

        req = urllib2.Request(self.url, data=body, headers=headers)
        response = urllib2.urlopen(req)
        doc = minidom.parseString(response.read())
        # root -> soap:Envelope -> soap:Body -> DispatchResponse -> s-gensym3
        id = doc.childNodes[0].childNodes[0].childNodes[0].childNodes[0].childNodes[0].toxml().encode('utf-8')
        return "%s/index.pl?Action=AgentTicketZoom;TicketID=%s" % (self.url.replace('/rpc.pl', ''), id)

    def _lookup_otrs(self, irc, msg):
        (recipients, text) = msg.args
        ticket_number = re.sub(".*%s([0-9]+).*" % self.prefix, r"\1", text)
        irc.reply(self._ticket_id(ticket_number), prefixNick=False)

    def doPrivmsg(self, irc, msg):
        (recipients, text) = msg.args
        if re.match(".*%s[0-9].*" % self.prefix, text):
            self._lookup_otrs(irc, msg)
        else:
            pass


Class = Otrs

# vim:set shiftwidth=4 softtabstop=4 expandtab:
