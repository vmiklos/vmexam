# first run /usr/lib/openoffice.org/program/soffice -silent -invisible -accept="socket,port=8100;urp;"
from OOoLib import *
import os
cSourceFile = os.path.abspath('test.doc')
cSourceURL = pathnameToUrl( cSourceFile )
cTargetFile = os.path.abspath('test.pdf')
cTargetURL = pathnameToUrl( cTargetFile )
oDoc = openURL( cSourceURL )
oDoc.storeToURL( cTargetURL, (createPropertyValue("FilterName","writer_pdf_Export"),) )
oDoc.dispose()
