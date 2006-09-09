__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Sun, 10 Sep 2006 00:25:02 +0200"
__copyright__ = "Copyright (c) 2006 Miklos Vajna"
__license__ = "GPL"

from sgmllib import SGMLParser
import urllib, string, sys, os

class BaseHTMLProcessor(SGMLParser):
	def reset(self):
		self.pieces = []
		SGMLParser.reset(self)
		self.inqt = False
		self.prevbr = False
	
	def wrap(self, text, width):
		return reduce(lambda line, word, width=width: '%s%s%s' %
					  (line,
					   ' \n'[(len(line)-line.rfind('\n')-1
							 + len(word.split('\n',1)[0]
								  ) >= width)],
					   word),
					  text.split(' ')
					 )

	def unknown_starttag(self, tag, attrs):
		if tag == "a":
			if attrs[0][1][:2] == "qt":
				self.inqt = True
		if tag == "hr":
			if self.inqt:
				self.pieces.append("%\n")
			self.inqt = False
		if tag == "br":
			if self.inqt and self.pieces[-1] != "\n":
				self.pieces.append("\n")
			if self.prevbr == False:
				self.prevbr = True
			else:
				if self.inqt:
					self.pieces.append("%\n")
				self.inqt = False
		else:
			self.prevbr = False

	def handle_data(self, text):
		if text.strip() != "" and self.inqt:
			self.pieces.append(text.replace("\n", " "))
	
	def output(self):
		return(self.wrap("".join(self.pieces), 80))

if __name__ == "__main__":
	if len(sys.argv) < 2:
		print "usage: %s id out_file" % sys.argv[0]
		sys.exit(1)
	id = sys.argv[1]
	out_file = sys.argv[2]
	out = open(out_file, "w")
	socket = urllib.urlopen("http://www.imdb.com/title/tt%s/quotes" % id)
	parser = BaseHTMLProcessor()
	parser.feed("".join(socket.readlines()))
	parser.close()
	socket.close()
	out.write(parser.output())
	out.close()
	os.system("strfile %s" % out_file)
