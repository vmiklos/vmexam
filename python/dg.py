#!/usr/bin/env python

import sys, tty, termios, os, re, getopt

class File:
	def __init__(self):
		self.header = None
		self.hunks = []
		self.new = False

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
		print "%s" % s
	else:
		print "bug in darcs-git!"
	print "at %s:%d" % inspect.stack()[1][1:3]

def get_diff(files = ""):
	sock = os.popen("git diff HEAD %s" % files)
	lines = sock.readlines()
	sock.close()
	return lines

def scan_dir(files=""):
	ret = []
	lines = get_diff(files)

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
			file = File()
			inheader = True
			header.append(i)
		elif i.startswith("+++"):
			header.append(i)
		elif i.startswith("---"):
			header.append(i)
			if i == "--- /dev/null\n":
				file.new = True
		elif i.startswith("@@"):
			if inheader:
				inheader = False
				file.header = "".join(header)
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

def askhunks(hunks, preans=None, action="record"):
	total = len(hunks)
	hunknum = 0
	commit = False
	for i in hunks:
		if preans == None:
			while True:
				sys.stdout.write(i.text)
				ret = ask("Shall I %s this change? (%d/%d)  [ynqad], or ? for help:" % (action, hunknum+1, total))
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
					print """How to use %(action)s...
y: %(action)s this patch
n: don't %(action)s it

d: %(action)s selected patches, skipping all the remaining patches
a: %(action)s all the remaining patches
q: cancel %(action)s

h or ?: show this help""" % { 'action': action }
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

def diff2filename(diff):
	return re.sub(r".* a/([^ ]+) .*", r"\1", diff)

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
	# a list for new files. we'll revert their addition, commit and add
	# them again
	newlist = []
	for i in status.hunks:
		if not i.picked:
			lines = i.text.split("\n")
			if "--- /dev/null" in lines:
				newlist.append(diff2filename(lines[0]))
	for i in newlist:
		os.system("git reset HEAD %s" % i)
	os.system("git commit -m '%s' %s" % (msg, opts))
	# readd the uncommitted new files
	for i in newlist:
		os.system("git add %s" % i)

def revert_stale():
	"""revert changes when only the modification date is changed. returns
	True if we did something"""
	ret = False
	lines = get_diff()
	prevdiff = False
	linenum = 0
	for i in lines:
		if i.startswith("diff "):
			if prevdiff:
				os.system("git checkout %s" % diff2filename(lines[linenum-1]))
				ret = True
			prevdiff = True
		else:
			prevdiff = False
		linenum += 1
	if prevdiff:
		os.system("git checkout %s" % diff2filename(lines[linenum-1]))
		ret = True
	return ret

def revert(argv):
	def usage(ret):
		print """Usage: darcs-git revert [OPTION]... [FILE or DIRECTORY]...
Revert to the committed version (you may loose your work).

Options:
  -a            --all                    answer yes to all hunks
  -h            --help                   shows brief description of command and its arguments"""
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.all = None
			self.help = False
			self.files = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "ah", ["all", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-a", "--all"):
			options.all = True
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	# check if we have anything to revert
	lines = get_diff(options.files)
	if not len(lines):
		print "There are no changes to revert!"
		sys.exit(0)
	if options.all:
		os.system("git checkout -f")
		print "Finished reverting."
		sys.exit(0)
	status = scan_dir(options.files)
	status.hunks = askhunks(status.hunks, action="revert")
	if not status.hunks:
		if revert_stale():
			print "Finished reverting."
		else:
			print "Ok, if you don't want to revert anything, that's fine!"
		sys.exit(0)
	for i in status.hunks:
		p = []
		if i.picked == True:
			p.append(i.text)
		sock = os.popen("patch -p1 -R >/dev/null", "w")
		sock.write("".join(p))
		sock.close()
	# we need git reset too if we revert deleted files
	for i in status.hunks:
		lines = i.text.split("\n")
		if "+++ /dev/null" in lines:
			os.system("git reset HEAD %s >/dev/null" % diff2filename(lines[0]))
	revert_stale()
	print "Finished reverting."

def whatsnew(argv):
	def usage(ret):
		print """Usage: darcs-git whatsnew [OPTION]... [FILE or DIRECTORY]...
Display uncommitted changes in the working directory.

Options:
  -s  --summary             summarize changes
  -h  --help                shows brief description of command and its arguments"""
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.summary = ""
			self.help = False
			self.files = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "sh", ["summary", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-s", "--summary"):
			options.summary = "--name-status"
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	ret = os.system("git diff HEAD -M --exit-code %s %s" % (options.summary, options.files))
	if not ret:
		print "No changes!"

def main(argv):
	if len(sys.argv) == 1:
		print "usage()"
	else:
		if sys.argv[1][:3] == "rec":
			record(argv[1:])
		elif sys.argv[1][:3] == "rev":
			revert(argv[1:])
		elif sys.argv[1][:4] == "what":
			whatsnew(argv[1:])
		else:
			os.system("git %s" % " ".join(argv))

if __name__ == "__main__":
	main(sys.argv[1:])
