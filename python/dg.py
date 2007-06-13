#!/usr/bin/env python

import sys, tty, termios, os, re, getopt

class File:
	def __init__(self, filename):
		if not filename:
			bug("filename can't be None!")
		self.filename = filename
		self.header = None
		self.hunks = []

class FileHunk:
	def __init__(self, hunk, picked=False):
		self.text = hunk
		self.picked = picked

class Files:
	def __init__(self, l):
		self.files = l
		self.hunks = []
		for i in self.files:
			for j in i.hunks:
				self.hunks.append(FileHunk(i.header + j))

def ask(s, type=None):
	sys.stdout.write("%s " % s)
	if type == str:
		try:
			return sys.stdin.readline().strip()
		except KeyboardInterrupt:
			print "Interrupted!"
			sys.exit(0)
	else:
		fd = sys.stdin.fileno()
		old_settings = termios.tcgetattr(fd)
		try:
			tty.setraw(sys.stdin.fileno())
			c = sys.stdin.read(1)
		finally:
			termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
		print c
		return c

def bug(s=None):
	import inspect
	if s:
		print "%s"
	else:
		print "bug in darcs-git!"
	print "at %s:%d" % inspect.stack()[1][1:3]

def scan_dir(files=""):
	ret = []
	sock = os.popen("git diff HEAD %s" % files)
	lines = sock.readlines()
	sock.close()

	inheader = False
	inhunk = False
	header = []
	hunk = []
	file = None
	for i in lines:
		if i.startswith("#"):
			continue
		elif i.startswith("diff"):
			if inhunk:
				file.hunks.append("".join(hunk))
				hunk = []
				inhunk = False
			if file:
				ret.append(file)
			file = File(re.sub(r".* a/([^ ]+) .*\n", r"\1", i))
			inheader = True
			header.append(i)
		elif i.startswith("+++"):
			header.append(i)
		elif i.startswith("---"):
			header.append(i)
		elif i.startswith("@@"):
			if inheader:
				inheader = False
				file.header = "".join(header)
				filename = None
				header = []
			if inhunk:
				file.hunks.append("".join(hunk))
				hunk = []
			inhunk = True
			hunk.append(i)
		elif i[0] == "+" or i[0] == "-" or i[0] == " ":
			if inhunk:
				hunk.append(i)
			else:
				bug("expected to be in a hunk")
	if inhunk:
		file.hunks.append("".join(hunk))
		hunk = []
		inhunk = False
	if file:
		ret.append(file)
	return Files(ret)

def askhunks(hunks, preans=None):
	total = len(hunks)
	hunknum = 0
	commit = False
	for i in hunks:
		if preans == None:
			while True:
				sys.stdout.write(i.text)
				ret = ask("Shall I record this change? (%d/%d)  [ynqad], or ? for help:" % (hunknum+1, total))
				if ret == "y":
					commit = True
					hunks[hunknum].picked = True
					break
				if ret == "n":
					hunks[hunknum].picked = False
					break
				if ret == "a":
					commit = True
					preans = True
					break
				if ret == "d":
					preans = False
					break
				if ret == "q":
					sys.exit(0)
					break
				if ret == "?" or ret == "h":
					print """How to use record...
y: record this patch
n: don't record it

d: record selected patches, skipping all the remaining patches
a: record all the remaining patches
q: cancel record

h or ?: show this help"""
				print "Invalid response, try again!"
		if preans != None:
			if preans == True:
				commit = True
			hunks[hunknum].picked = preans
		hunknum += 1
	if commit == False:
		return commit
	else:
		return hunks


def record(argv):
	def usage(ret):
		print """Usage: darcs-git record [OPTION]... [FILE or DIRECTORY]...
Save changes in the unstaged index to the current branch as a commit.

Options:
  -m PATCHNAME  --commit-name=PATCHNAME  name of commit
  -a            --all                    answer yes to all hunks
  -e            --edit-long-comment      backward compatibility
  -h            --help                   shows brief description of command and its arguments"""
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.name = None
			self.all = None
			self.help = False
			self.files = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "m:ah", ["commit-name=", "all", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-m", "--commit-name"):
			options.name = arg
		elif opt in ("-a", "--all"):
			options.all = True
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	status = scan_dir(options.files)
	status.hunks = askhunks(status.hunks, options.all)
	if status.hunks:
		if options.name:
			msg = options.name
		else:
			msg = ask("What is the patch name?", str)
	else:
		print "Ok, if you don't want to record anything, that's fine!"
		sys.exit(0)
	flist = []
	for i in status.hunks:
		if i.picked:
			flist.append(re.sub(r".* a/([^ ]+) .*", r"\1", i.text.split("\n")[0]))
	while True:
		ret = ask("Do you want to add a long comment? [ynq]")
		if ret == "y":
			opts = "-e"
			break
		if ret == "n":
			opts = ""
			break
		if ret == "q":
			sys.exit(0)
		print "Invalid response, try again!"
	for i in status.hunks:
		p = []
		if i.picked == True:
			p.append(i.text)
		sock = os.popen("git apply --cached 2>/dev/null", "w")
		sock.write("".join(p))
		sock.close()
	os.system("git commit -m '%s' %s %s" % (msg, opts, " ".join(flist)))

def main(argv):
	if len(sys.argv) == 1:
		print "usage()"
	else:
		if sys.argv[1][:3] == "rec":
			record(argv[1:])
		else:
			os.system("git %s" % " ".join(argv))

if __name__ == "__main__":
	main(sys.argv[1:])
