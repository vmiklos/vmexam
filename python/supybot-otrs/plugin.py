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
        self.prefixes = config.get(self.url, "prefixes").split(",")

        self.bugzillaUrl = None
        if len(config.sections()) > 1:
            self.bugzillaUrl = config.sections()[1]
            self.bugzillaPrefixes = config.get(self.bugzillaUrl, "prefixes").split(",")

    def _ticket_id(self, number):
        #
        # Example ~/.otrsrc:
        #
        # [https://localhost/otrs/rpc.pl]
        # soap_user = mycompany
        # soap_password = secret
        # prefixes = bug#,otrs#
        #
        # [https://localhost/bugzilla/show_bug.cgi]
        # prefixes = bz#
        #
        # The second section is optional.
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

    def _lookup_otrs(self, irc, msg, prefix):
        (recipients, text) = msg.args
        ticket_number = re.sub(".*%s([0-9]+).*" % prefix, r"\1", text)
        irc.reply(self._ticket_id(ticket_number), prefixNick=False)

    def _lookup_bugzilla(self, irc, msg, prefix):
        (recipients, text) = msg.args
        ticket_number = re.sub(".*%s([0-9]+).*" % prefix, r"\1", text)
        irc.reply("%s?id=%s" % (self.bugzillaUrl, ticket_number), prefixNick=False)

    def doPrivmsg(self, irc, msg):
        (recipients, text) = msg.args
        for prefix in self.prefixes:
            if re.match(".*%s[0-9].*" % prefix, text):
                self._lookup_otrs(irc, msg, prefix)
        if self.bugzillaUrl:
            for prefix in self.bugzillaPrefixes:
                if re.match(".*%s[0-9].*" % prefix, text):
                    self._lookup_bugzilla(irc, msg, prefix)


Class = Otrs

# vim:set shiftwidth=4 softtabstop=4 expandtab:
