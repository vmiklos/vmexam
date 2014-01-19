#!/usr/bin/env python3

import sys
import time
import uno


def main():
    # Extract parameters
    connectionString = "uno:socket,host=localhost,port=2083;urp;StarOffice.ServiceManager"
    count = len(sys.argv) - 1
    if count < 1:
        print("usage: test.py <file_url> [<uno_connection_url>]")
        print("example: test.py \"file:///e:/temp/test.odt\"")
        return 1
    docUrl = sys.argv[1]
    if count == 2:
        connectionString = sys.argv[2]

    # Initialize UNO: result is the xComponentContext what we need to create any service.
    xComponentContext = uno.getComponentContext()
    xResolver = xComponentContext.ServiceManager.createInstanceWithContext("com.sun.star.bridge.UnoUrlResolver", xComponentContext)
    xComponentContext = xResolver.resolve(connectionString).DefaultContext

    # Load the document: create the frame::Desktop service and load the document.
    xDesktop = xComponentContext.ServiceManager.createInstanceWithContext("com.sun.star.frame.Desktop", xComponentContext)
    startTime = int(time.time() * 1000)
    xComponent = xDesktop.loadComponentFromURL(docUrl, "_blank", 0, ())
    endTime = int(time.time() * 1000)
    print("loadComponentFromURL() finished in %d ms" % (endTime - startTime))

if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
