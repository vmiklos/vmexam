#!/usr/bin/env python3
# 
#   darcs-git, a darcs-like porcelain on top of git plumbing
#  
#   Copyright (c) 2007, 2008, 2009, 2010, 2011 by Miklos Vajna <vmiklos@frugalware.org>
#  
#   This program is free software; you can redistribute it and/or modify
#   it under the terms of the GNU General Public License as published by
#   the Free Software Foundation; either version 2 of the License, or
#   (at your option) any later version.
# 
#   This program is distributed in the hope that it will be useful,
#   but WITHOUT ANY WARRANTY; without even the implied warranty of
#   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#   GNU General Public License for more details.
#  
#   You should have received a copy of the GNU General Public License
#   along with this program; if not, write to the Free Software
#   Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, 
#   USA.
#

__version__ = "0.7"

import sys, tty, termios, os, re, getopt, hashlib

class File:
	def __init__(self):
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
	def ispicked(self, hunk):
		needle = diff2filename(hunk.text.split("\n")[0])
		for i in self.hunks:
			if needle == diff2filename(i.text.split("\n")[0]):
				return i.picked

def ask(s, type=None):
	sys.stdout.write("%s " % s)
	sys.stdout.flush()
	if type == str:
		try:
			return sys.stdin.readline().strip()
		except KeyboardInterrupt:
			print("Interrupted!")
			sys.exit(0)
	else:
		fd = sys.stdin.fileno()
		old_settings = termios.tcgetattr(fd)
		try:
			tty.setraw(sys.stdin.fileno())
			c = sys.stdin.read(1)
		finally:
			termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
		print(c)
		return c

def bug(s=None):
	import inspect
	if s:
		print("%s" % s)
	else:
		print("bug in darcs-git!")
	print("at %s:%d" % inspect.stack()[1][1:3])

def emptydir(dir):
	ret = True
	for root, dirs, files in os.walk(dir):
		for file in files:
			ret = False
			break
		if not ret:
			break
	return ret

def get_root():
	sock = os.popen("git rev-parse --git-dir")
	root = sock.read().strip()
	if sock.close():
		sys.exit(0)
	return root

def get_branch():
	sock = os.popen("git symbolic-ref HEAD")
	branch = sock.read().strip()[11:]
	if sock.close():
		sys.exit(0)
	return branch

def get_merge(branch):
	sock = os.popen("git config branch.%s.merge" % branch)
	merge = sock.read().strip()[11:]
	if sock.close():
		sys.exit(0)
	return merge

def get_remote(branch):
	sock = os.popen("git config branch.%s.remote" % branch)
	remote = sock.read().strip()
	if sock.close():
		sys.exit(0)
	return remote

def get_diff(files = ""):
	sock = os.popen("git diff HEAD --binary %s" % files)
	lines = sock.readlines()
	sock.close()
	if len(lines) and lines[0].startswith("[1m"):
		print("""It seems that you force using colors in your diffs
which is not compatible with darcs-git. Please set that value
to false or auto. Example:

git config diff.color auto""")
		sys.exit(0)
	return lines

def merge_check():
	ret = False
	sock = os.popen("git diff")
	lines = sock.readlines()
	sock.close()
	for i in lines:
		if i.startswith("diff --cc "):
			ret = True
			break
	return ret

def svn_check():
	sock = os.popen("git rev-parse --show-cdup")
	cdup = sock.read().strip()
	sock.close()
	return os.path.exists(os.path.join(cdup, ".git/svn"))

def svn_get_remote():
	sock = os.popen("git config svn-remote.svn.fetch")
	# Returns something like ':refs/remotes/origin/master'.
	fetch = sock.readline().strip()
	sock.close()
	if not len(fetch):
		return "git-svn"
	else:
		return fetch[1:]

def darcs_check():
	sock = os.popen("git rev-parse --show-cdup")
	cdup = sock.read().strip()
	sock.close()
	return os.path.exists(os.path.join(cdup, ".git/darcs"))

def scan_dir(files=""):
	ret = []
	lines = get_diff(files)

	inheader = False
	inhunk = False
	header = []
	hunk = []
	file = None
	binary = False
	for i in lines:
		if i.startswith("#"):
			continue
		elif i.startswith("diff"):
			binary = False
			if inhunk:
				file.hunks.append("".join(hunk))
				hunk = []
				inhunk = False
			if file:
				ret.append(file)
			file = File()
			inheader = True
			header.append(i)
		elif i.startswith("+++ ") or i.startswith("--- ") or i.startswith("index ") or i.startswith("deleted "):
			header.append(i)
		elif i.startswith("@@") or i.startswith("GIT binary patch"):
			if i.startswith("GIT binary patch"):
				binary = True
			if inheader:
				inheader = False
				file.header = "".join(header)
				header = []
			if inhunk:
				file.hunks.append("".join(hunk))
				hunk = []
			inhunk = True
			hunk.append(i)
		elif i[0] == "+" or i[0] == "-" or i[0] == " " or binary:
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
				sys.stdout.flush()
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
					print("""How to use %(action)s...
y: %(action)s this patch
n: don't %(action)s it

d: %(action)s selected patches, skipping all the remaining patches
a: %(action)s all the remaining patches
q: cancel %(action)s

h or ?: show this help""" % { 'action': action })
				print("Invalid response, try again!")
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
	return re.sub(r".* [a-z]/([^ ]+) .*", r"\1", diff)

def record(argv, amend=False):
	def usage(ret):
		print("""Usage: darcs-git record [OPTION]... [FILE or DIRECTORY]...
Save changes in the unstaged index to the current branch as a commit.

Options:
  -m PATCHNAME  --commit-name=PATCHNAME  name of commit
  -a            --all                    answer yes to all hunks
  -e            --edit-long-comment      backward compatibility
  -s            --skip-long-comment      Don't give a long comment
  -h            --help                   shows brief description of command and its arguments""")
		sys.exit(ret)

	class Options:
		def __init__(self, amend):
			self.name = None
			self.all = None
			self.edit = None
			self.help = False
			self.files = ""
			if amend:
				self.amend = "--amend"
			else:
				self.amend = ""
	options = Options(amend)

	try:
		opts, args = getopt.getopt(argv, "m:aesh", ["commit-name=", "all", "edit-long-comment", "skip-long-comment", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-m", "--commit-name"):
			options.name = arg
			optind += 1
		elif opt in ("-e", "--edit-long-comment"):
			options.edit = "-e"
		elif opt in ("-s", "--skip-long-comment"):
			options.edit = ""
		elif opt in ("-a", "--all"):
			options.all = True
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	root = get_root()
	first = False
	if os.system("git rev-parse --verify HEAD >/dev/null 2>&1"):
		first = True
		sock = os.popen("git status")
		lines = sock.readlines()
		sock.close()
		changes = True
		for i in lines:
			if i.startswith("nothing"):
				changes = False
		if changes:
			print("This is a new repo, can't cherry-pick for the first commit.")
		else:
			print("No changes!")
			sys.exit(0)
	merge = False
	if merge_check():
		print("This is a merge, can't cherry-pick for this commit.")
		merge = True
	if first or merge:
		status = Files([])
	else:
		# we need the overall status too, to exclude new files if necessary
		allstatus = scan_dir()
		status = scan_dir(options.files)
		if not options.all:
			status.hunks = askhunks(status.hunks)
	if first or merge or status.hunks:
		if not options.name:
			options.name = ask("What is the patch name?", str)
	else:
		print("Ok, if you don't want to record anything, that's fine!")
		sys.exit(0)
	if options.edit is None:
		while True:
			ret = ask("Do you want to add a long comment? [ynq]")
			if ret == "y":
				options.edit = "-e"
				break
			if ret == "n":
				options.edit = ""
				break
			if ret == "q":
				sys.exit(0)
			print("Invalid response, try again!")
	root = os.path.split(get_root())[0]
	if len(root):
		os.chdir(root)
	if first or merge or options.all:
		os.system("""git commit -a -m "%s" %s %s %s""" %
				(options.name.replace('"', r'\"'), options.edit, options.amend, options.files))
		sys.exit(0)
	for i in status.hunks:
		p = []
		if i.picked == True:
			p.append(i.text)
		sock = os.popen("git apply --cached 2>/dev/null", "w")
		sock.write("".join(p))
		sock.close()
	# a list for new/deleted files. we'll revert their addition/deletion,
	# commit and add/remove them again
	newlist = []
	dellist = []
	for i in allstatus.hunks:
		if not status.ispicked(i):
			lines = i.text.split("\n")
			new = False
			old = False
			for j in lines:
				if j.startswith("index 0000000"):
					new = True
					break
				elif re.match(r"index [0-9a-f]{7}\.\.0000000", j):
					old = True
					break
			if new:
				newlist.append(diff2filename(lines[0]))
			if old:
				dellist.append(i.text)
		else:
			lines = i.text.split("\n")
			new = False
			for j in lines:
				if j.startswith("index 0000000"):
					new = True
					break
			if new:
				# this is a newly added file but maybe it has
				# been updated since add. add it again
				os.system("git add %s" % diff2filename(lines[0]))
	for i in newlist:
		os.system("git reset -q HEAD %s" % i)
	for i in dellist:
		sock = os.popen("git apply --cached -R 2>/dev/null", "w")
		sock.write(i)
		sock.close()
	os.system("""git commit -m '%s' %s %s""" %
			(options.name.replace("'", """'"'"'"""), options.edit, options.amend))
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
		print("""Usage: darcs-git revert [OPTION]... [FILE or DIRECTORY]...
Revert to the committed version (you may loose your work).

Options:
  -a            --all                    answer yes to all hunks
  -h            --help                   shows brief description of command and its arguments""")
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
		print("There are no changes to revert!")
		sys.exit(0)
	if options.all:
		if(len(options.files)):
			os.system("git checkout %s" % options.files)
		else:
			os.system("git checkout -f")
		print("Finished reverting.")
		sys.exit(0)
	status = scan_dir(options.files)
	status.hunks = askhunks(status.hunks, action="revert")
	if not status.hunks:
		if revert_stale():
			print("Finished reverting.")
		else:
			print("Ok, if you don't want to revert anything, that's fine!")
		sys.exit(0)
	root = os.path.split(get_root())[0]
	if len(root):
		os.chdir(root)
	for i in status.hunks:
		p = []
		if i.picked == True:
			p.append(i.text)
		sock = os.popen("git apply -R 2>/dev/null", "w")
		sock.write("".join(p))
		sock.close()
	# we need git reset too if we revert deleted files
	for i in status.hunks:
		lines = i.text.split("\n")
		new = False
		for j in lines:
			if j.startswith("index 0000000"):
				new = True
				break
		if new:
			os.system("git reset -q HEAD %s" % diff2filename(lines[0]))
	revert_stale()
	print("Finished reverting.")

def whatsnew(argv):
	def usage(ret):
		print("""Usage: darcs-git whatsnew [OPTION]... [FILE or DIRECTORY]...
Display uncommitted changes in the working directory.

Options:
  -s  --summary             summarize changes
  -h  --help                shows brief description of command and its arguments""")
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.summary = ""
			self.help = False
			self.files = ""
			self.head = "HEAD"
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
	if os.system("git rev-parse --verify HEAD >/dev/null 2>&1"):
		options.head = hashlib.sha1("tree 0\0").hexdigest()
	os.system("git update-index --refresh >/dev/null")
	ret = os.system("git diff %s -M -C --exit-code %s %s" % (options.head, options.summary, options.files))
	if not ret:
		print("No changes!")

def changes(argv):
	def usage(ret):
		print("""Usage: darcs-git changes [OPTION]... [FILE or DIRECTORY]...
Gives a changelog-style summary of the branch history.

Options:
  -l         --last=NUMBER         select the last NUMBER patches
  -s         --summary             summarize changes
  -v         --verbose             give verbose output
  -t         --tags                include tags in the log (darcs-git only)
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.last = ""
			self.logopts = ""
			self.help = False
			self.tags = ""
			self.abbrev = "--abbrev-commit --abbrev=7"
			self.files = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "l:svth", ["last=", "summary", "verbose", "tags", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-l", "--last"):
			options.last = "-%s" % arg
			optind += 1
		elif opt in ("-s", "--summary"):
			options.logopts = "-r --name-status"
		elif opt in ("-v", "--verbose"):
			options.logopts = "-p"
		elif opt in ("-t", "--tags"):
			options.tags = "| git name-rev --tags --stdin"
			options.abbrev = ""
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	return os.system(" ".join(['git log -M',
		options.last, options.logopts, options.files, options.tags, options.abbrev]))

def push(argv):
	def usage(ret):
		print("""Usage: darcs-git push [OPTION]... [GIT OPTIONS]...
Copy and apply patches from this repository to another one.

Options:
  -a         --all                 answer yes to all questions
  -s         --summary             summarize changes
  -v         --verbose             give verbose output
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.all = False
			self.verbose = False
			self.summary = False
			self.help = False
			self.gitopts = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "asvh", ["all", "summary", "verbose", "help"])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-a", "--all"):
			options.all = True
		elif opt in ("-s", "--summary"):
			options.summary = True
		elif opt in ("-v", "--verbose"):
			options.verbose = True
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	branch = get_branch()
	if optind < len(argv):
		options.gitopts = " ".join(argv[optind:])
	else:
		options.gitopts = get_remote(branch)
	if options.help:
		usage(0)
	remote = "%s/%s" % (options.gitopts, get_merge(branch))
	if svn_check():
		remote = svn_get_remote()
	elif darcs_check():
		remote = "darcs/upstream"
	logopts = ""
	if options.verbose:
		logopts += "-p "
	if options.summary:
		logopts += "--name-status"
	sock = os.popen("git log %s %s..%s 2>&1" % (logopts, remote, branch))
	lines = sock.readlines()
	ret = sock.close()
	if not len(lines):
		print("No recorded local changes to push!")
		return 0
	print("".join(lines))
	if not options.all:
		while True:
			ret = ask("Do you want to push these patches? [ynq]")
			if ret == "y":
				break
			if ret in ("n", "q"):
				return(0)
			print("Invalid response, try again!")
	if svn_check():
		os.system("git svn dcommit")
	elif darcs_check():
		os.system("git darcs push upstream")
	else:
		ret = os.system("git push %s" % options.gitopts)
		if ret:
			ret = pull(['-a'])
			if not ret:
				ret = os.system("git push %s" % options.gitopts)
				if ret:
					return(1)
	return(0)

def pull(argv):
	def usage(ret):
		print("""Usage: darcs-git pull [OPTION]... [GIT OPTIONS]...
Copy and apply patches to this repository from another one.

Options:
  -a         --all                 answer yes to all questions
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.all = False
			self.help = False
			self.gitopts = ""
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
	branch = get_branch()
	if optind < len(argv):
		options.gitopts = " ".join(argv[optind:])
	else:
		options.gitopts = get_remote(branch)
	if options.help:
		usage(0)
	if svn_check():
		os.system("git svn fetch")
	elif darcs_check():
		os.system("git darcs fetch upstream")
	else:
		os.system("git fetch %s" % options.gitopts)
	remote = "%s/%s" % (options.gitopts, get_merge(branch))
	if svn_check():
		remote = svn_get_remote()
	elif darcs_check():
		remote = "darcs/upstream"
	sock = os.popen("git log %s..%s 2>&1" % (branch, remote))
	lines = sock.readlines()
	ret = sock.close()
	if not len(lines):
		print("No remote changes to pull!")
		return 0
	print("".join(lines))
	if not options.all:
		while True:
			ret = ask("Do you want to pull these patches? [ynq]")
			if ret == "y":
				break
			if ret in ("n", "q"):
				return(0)
			print("Invalid response, try again!")
	if os.system("git diff-index --quiet --cached HEAD && git diff-files --quiet") != 0:
		changes = True
		if os.system("git stash") != 0:
			return(1)
	else:
		changes = False
	if svn_check():
		if os.system("git svn rebase -l") != 0:
			return(1)
	elif darcs_check():
		if os.system("git rebase darcs/upstream") != 0:
			return(1)
	else:
		if os.system("git pull --rebase %s" % options.gitopts) != 0:
			return(1)
	if changes and os.system("git stash pop") != 0:
			return(1)

def send(argv):
	def usage(ret):
		print("""Usage: darcs-git send [OPTION]... <PATCHES>
Send by email a bundle of one or more patches.

The recommended workflow is:

	1) darcs-git format-patch
	   Optionally you can now edit the patches to add custom headers like
	   In-Reply-To ones and/or custom message between --- and the diffstat.
	2) darcs-git send --to="M A Intener <m8r@example.com>" *.patch

Use "darcs-git help send-email" for more information.

Options:
  -d  --dry-run                      don't actually take the action
  -h  --help                         shows brief description of command and its arguments
  -t  --to                           specify destination EMAIL
  -c  --cc                           additional EMAIL(s).""")
		sys.exit(ret)
	
	class Options:
		def __init__(self):
			self.dryrun = ""
			self.help = False
			self.to = ""
			self.cc = ""
			self.gitopts = ""
	options = Options()

	try:
		opts, args = getopt.getopt(argv, "c:dt:", ["cc=", "dry-run", "to="])
	except getopt.GetoptError:
		usage(1)
	optind = 0
	for opt, arg in opts:
		if opt in ("-d", "--dry-run"):
			options.dryrun = "--dry-run"
		elif opt in ("-h", "--help"):
			options.help = True
		elif opt in ("-t", "--to"):
			for i in arg.split(', '):
				options.to += ' --to="%s"' % i.replace('"', r'\"')
		elif opt in ("-c", "--cc"):
			for i in arg.split(', '):
				options.cc += ' --cc="%s"' % i.replace('"', r'\"')
		optind += 1
	if optind < len(argv):
		options.gitopts = " ".join(argv[optind:])
	if options.help:
		usage(0)
	sock = os.popen("git config user.name")
	author = sock.readline().strip()
	sock.close()
	sock = os.popen("git config user.email")
	author += " <%s>" % sock.readline().strip()
	sock.close()
	return os.system("""git send-email --envelope-sender "%s" --from "%s" --suppress-from %s %s %s %s""" % (author, author, options.dryrun, options.to, options.cc, options.gitopts))

def get(argv):
	def usage(ret):
		print("""Usage: darcs-git get [OPTION]... <REPOSITORY> [<DIRECTORY>]
Create a local copy of another repository.
Use "darcs-git help clone" for more information.

Options:
  -h  --help                         shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	ret = os.system("git clone --recursive %s" % " ".join(argv))
	if ret:
		return ret

def setpref(argv):
	def usage(ret):
		print("""Usage: darcs-git setpref [OPTION]...
Set a value for a preference.
Use "darcs-git help config" for more information.

Options:
  -h  --help                         shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	return os.system("git config %s" % " ".join(argv))

def tag(argv):
	def usage(ret):
		print("""Usage: darcs-git tag [PROJECTNAME] <VERSION>
Tag the contents of the repository with a version name.
Use "darcs-git help tag" for more information.

Options:
  -h  --help                         shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	ret = 0
	ret += os.system("git commit --allow-empty -m 'TAG %s'" % argv[-1])
	if len(argv) > 1:
		msg = " ".join(argv[:2])
	else:
		msg = argv[0]
	ret += os.system("git tag -a -m '%s' '%s'" % (msg, argv[-1]))
	if ret:
		os.system("git reset --hard HEAD^")
	return ret

def rollback(argv):
	def usage(ret):
		print("""Usage: darcs-git rollback [OPTION]... <COMMIT-HASH>
Commit an inverse patch.
Use "darcs-git help revert" for more information.

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	return os.system("git revert %s" % " ".join(argv))

def unrecord(argv):
	def usage(ret):
		print("""Usage: darcs-git unrecord [OPTION]...
Remove last committed patch without changing the working directory.
This is an alias for "git reset -q HEAD^".

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	while True:
		os.system("git log -1")
		ret = ask("Do you want to unrecord this patch? [ynq]")
		if ret == "y":
			break
		if ret in ("n", "q"):
			sys.exit(0)
		print("Invalid response, try again!")
	os.system("git reset -q HEAD^ %s >/dev/null" % " ".join(argv))
	print("Finished unrecording.")

def unpull(argv):
	def usage(ret):
		print("""Usage: darcs-git unpull [OPTION]...
Opposite of pull; unsafe if the latest patch is not in remote repository.
This is an alias for "git reset --hard HEAD^".

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	while True:
		os.system("git log -1")
		ret = ask("Do you want to unpull this patch? [ynq]")
		if ret == "y":
			break
		if ret in ("n", "q"):
			sys.exit(0)
		print("Invalid response, try again!")
	os.system("git reset --hard HEAD^ %s" % " ".join(argv))
	print("Finished unpulling.")

def optimize(argv):
	def usage(ret):
		print("""Usage: darcs-git optimize [OPTION]...
Optimize the repository.
This is an alias for "git gc".

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	print("Checking how much disk space is wasted...")
	os.system("git count-objects")
	print("Cleaning up...")
	os.system("git gc")

def query(argv):
	def usage(ret):
		print("""Usage: darcs-git query SUBCOMMAND ...
Query information which is stored by darcs.

Subcommands:

  manifest      List version-controlled files in the working copy.
  tags          List all tags in the repository.

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	if len(argv) and argv[0] == "manifest":
		return os.system("git ls-files")
	elif len(argv) and argv[0] == "tags":
		return os.system("git tag -l")
	else:
		print("Invalid subcommand!")
		usage(1)

def check(argv):
	def usage(ret):
		print("""Usage: darcs-git check [OPTION]...
Check the repository for consistency.
This is an alias for "git fsck".

Options:
  -h         --help                shows brief description of command and its arguments""")
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	os.system("git fsck")

def main(argv):
	def usage(ret):
		os.system("man darcs-git")
		sys.exit(ret)
	def version():
		print("""darcs-git (pacman-tools) %s

Copyright (c) 2007 by Miklos Vajna <vmiklos@frugalware.org>
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR \
PURPOSE.""" % __version__)
	if len(sys.argv) == 1 or sys.argv[1] in ["-h", "--help"]:
		usage(0)
	if sys.argv[1] in ["-v", "--version"]:
		version()
	else:
		# this will exit if no root found
		if sys.argv[1] not in ["init", "get"]:
			get_root()
		if sys.argv[1][:4] != "chan":
			os.environ['GIT_PAGER'] = 'cat'
		if sys.argv[1][:3] == "rec":
			return record(argv[1:])
		elif sys.argv[1] == "amend-record":
			return record(argv[1:], amend=True)
		elif sys.argv[1][:3] == "rev":
			return revert(argv[1:])
		elif sys.argv[1][:4] == "what":
			return whatsnew(argv[1:])
		elif sys.argv[1][:4] == "chan":
			return changes(argv[1:])
		elif sys.argv[1] == "push":
			return push(argv[1:])
		elif sys.argv[1] == "pull":
			return pull(argv[1:])
		elif sys.argv[1] == "send":
			return send(argv[1:])
		elif sys.argv[1] == "setpref":
			return setpref(argv[1:])
		elif sys.argv[1] == "get":
			return get(argv[1:])
		elif sys.argv[1] == "tag":
			return tag(argv[1:])
		elif sys.argv[1][:4] == "roll":
			return rollback(argv[1:])
		elif sys.argv[1][:5] == "unrec":
			return unrecord(argv[1:])
		elif sys.argv[1] == "unpull":
			return unpull(argv[1:])
		elif sys.argv[1] == "obliterate":
			return unpull(argv[1:])
		elif sys.argv[1][:3] == "opt":
			return optimize(argv[1:])
		elif sys.argv[1] == "check":
			return check(argv[1:])
		elif sys.argv[1] == "query":
			return query(argv[1:])
		else:
			return os.system("git '%s'" % "' '".join(argv))

if __name__ == "__main__":
	if main(sys.argv[1:]) != 0:
		sys.exit(1)
	else:
		sys.exit(0)
