#!/usr/bin/env python

"""Converts xchat logs to irssi format

Usage: python xchat2irssi.py [options]

Options:
  -f ..., --from=...      encoding of the xchat logs [Defaults to utf-8]
  -t ..., --to=...        encoding of the irssi logs [Defaults to iso-8859-2]
  -i ..., --input=...     directory where the xchat logs are located [Defaults to ~/.xchat2/xchatlogs]
  -o ..., --output=...    directory where the irssi logs will be located [Defaults to ~/.irssi/logs]

Limitations (because xchat does not log them):
  - On quit, the hostname is not displayed.
  - Topic changes are not logged.

At least irssistats worked fine with the converted logs.
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Sat, 12 Aug 2006 01:53:48 +0200"
__copyright__ = "Copyright (c) 2006 Miklos Vajna"
__license__ = "GPL"

import getopt, re, os, time, sys

class x2ilogs:
	def __init__(self):
		self.ienc = 'utf-8'
		self.oenc = 'iso-8859-2'
		self.idir = os.path.join(os.environ['HOME'], ".xchat2/xchatlogs")
		self.odir = os.path.join(os.environ['HOME'], ".irssi/logs")

	def convertfile(self, fnorig, fn):
		try:
			fi = open('%s/%s' % (self.idir, fnorig))
		except IOError, e:
			print e
			print "Is your directory structure correct?"
			sys.exit()

		linenum = 1
		fo = None
		while True:
			try:
				line = unicode(fi.next(), self.ienc).encode(self.oenc)
			except UnicodeEncodeError:
				# xchat devs would have to learn how to use utf8
				print "WARNING: Failed to encode %s, line %d, skipping" % (fi.name, linenum)
				linenum += 1
				continue
			except UnicodeDecodeError:
				print "WARNING: Failed to decode %s, line %d, skipping" % (fi.name, linenum)
				linenum += 1
				continue
			except StopIteration:
				break
			else:
				linenum += 1
			if line == "\n" or re.match(r'.+ [0-9]+ [0-9][0-9]:[0-9][0-9]:[0-9][0-9] ---', line):
				continue
			elif line[:21] == "**** BEGIN LOGGING AT":
				# xchat uses some random.choice()..
				line = line.replace("AT:", "AT")
				try:
					logtime = time.strptime(line[22:-1])
				except ValueError:
					raise "Invalid date format in file %s, line %d" % (fi.name, linenum)
				if fo:
					fo.close()
				try:
					fn.index("#")
				except ValueError:
					network = fn.split('-')[0].lower()
					chan = fn.split('-')[1].strip(".log")
				else:
					network = fn.split('-#')[0].lower()
					chan = "#" + fn.split('-#')[1].strip(".log")
				dateformat = time.strftime("%Y%m%d",logtime)
				logname = "%s/%s/%s_%s.log" % (self.odir, network, chan, dateformat)
				try:
					os.mkdir("%s/%s" % (self.odir, network))
				except OSError:
					pass
				fo = open(logname, 'a')
				line = line.replace("**** BEGIN LOGGING AT", "--- Log opened")
			elif line[:22] == "**** ENDING LOGGING AT":
				line = line.replace("AT:", "AT")
				line = line.replace("**** ENDING LOGGING AT", "--- Log closed")
			elif re.match(r'.+ [0-9]+ [0-9][0-9]:[0-9][0-9]:[0-9][0-9] <.+>', line):
				# privmsg
				line = re.sub('^.+ [0-9]+ ([0-9][0-9]:[0-9][0-9]):[0-9][0-9] <(.+)>\t', r'\1 < \2> ', line)
			elif re.match(r'.+ [0-9]+ [0-9][0-9]:[0-9][0-9]:[0-9][0-9] (-->|<--)', line):
				# join/part
				line = re.sub('^.+ [0-9]+ ([0-9][0-9]:[0-9][0-9]):[0-9][0-9] .-.\t', r'\1 -!- ', line)
				line = line.replace('(', '[').replace(')', ']')
			elif re.match(r'.+ [0-9]+ [0-9][0-9]:[0-9][0-9]:[0-9][0-9] \*', line):
				# action
				line = re.sub('^.+ [0-9]+ ([0-9][0-9]:[0-9][0-9]):[0-9][0-9] \\*\t', r'\1 * ', line)
			try:
				fo.write(line)
			# eh, and xchat does not log the topic changes..
			except AttributeError:
				raise "The log %s, line %d has an invalid format!" % (fi.name, linenum)
		fi.close()
		fo.close()

	def convert(self):
		for root, dirs, files in os.walk(self.idir):
			for file in files:
				self.convertfile(file, unicode(file, self.ienc).encode(self.oenc))

def usage():
	print __doc__

def main(argv):
	# option parsing
	try:
		opts, args = getopt.getopt(argv, "f:t:i:o:", ["from=", "to=", "input=", "output="])
	except getopt.GetoptError:
		usage()
		sys.exit(1)
	logs = x2ilogs()
	for opt, arg in opts:
		if opt in ("-f", "--from"):
			logs.ienc = arg
		elif opt in ("-t", "--to"):
			logs.oenc = arg
		elif opt in ("-i", "--input"):
			logs.idir = arg
		elif opt in ("-o", "--output"):
			logs.odir = arg
	# the fun part
	logs.convert()

if __name__ == "__main__":
	main(sys.argv[1:])
