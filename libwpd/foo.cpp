#include <libwpd/WPXStreamImplementation.h>
#include <libwpd/WPDocument.h>

#include <iostream>

using namespace std;

int main()
{
	char szInputFile[] = "wp_test_document.wpd";
	WPXInputStream* input = new WPXFileStream(szInputFile);
	WPDConfidence confidence = WPDocument::isFileFormatSupported(input, true);
	if(confidence == WPD_CONFIDENCE_EXCELLENT)
		cout << "good" << endl;
	else
		cout << "bad" << endl;
}
