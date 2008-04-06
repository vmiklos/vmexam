#include <stdio.h>
#include <string.h>
#include <libxml/xmlwriter.h>

int main()
{
	xmlTextWriterPtr writer;
	xmlChar *ptr;

	writer = xmlNewTextWriterFilename("out.xml", 0);
	xmlTextWriterSetIndentString(writer, "\t");
	xmlTextWriterStartDocument(writer, NULL, "UTF-8", NULL);
	xmlTextWriterStartElement(writer, BAD_CAST "config");
	xmlTextWriterSetIndent(writer, 1);
	xmlTextWriterStartElement(writer, BAD_CAST "options");
	xmlTextWriterSetIndent(writer, 2);
	xmlTextWriterWriteFormatElement(writer, BAD_CAST "ident_method", "%s", "pass");
	xmlTextWriterSetIndent(writer, 1);
	xmlTextWriterEndElement(writer);
	xmlTextWriterSetIndent(writer, 0);
	xmlTextWriterEndElement(writer);
	xmlTextWriterEndDocument(writer);
	xmlFreeTextWriter(writer);
}
