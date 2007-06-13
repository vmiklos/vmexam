#!/usr/bin/env python

import sys, tty, termios, os, re

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

def scan_dir():
	ret = []
	sock = os.popen("git status -v -a")
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

def askhunks(hunks):
	total = len(hunks)
	hunknum = 0
	preans = None
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
			hunks[hunknum].picked = preans
		hunknum += 1
	if commit == False:
		return commit
	else:
		return hunks


def record():
	status = scan_dir()
	status.hunks = askhunks(status.hunks)
	if status.hunks:
		pass
		msg = ask("What is the patch name?", str)
	else:
		print "Ok, if you don't want to record anything, that's fine!"
		sys.exit(0)
	for i in status.hunks:
		p = []
		if i.picked == True:
			p.append(i.text)
		sock = os.popen("git apply --cached 2>/dev/null", "w")
		sock.write("".join(p))
		sock.close()
	os.system("git commit -m '%s' -e" % msg)

def main():
	if len(sys.argv) == 1:
		print "usage()"
	else:
		if sys.argv[1] == "rec":
			record()
		else:
			print "usage()"

if __name__ == "__main__":
	main()
