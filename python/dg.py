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
	def ispicked(self, hunk):
		needle = diff2filename(hunk.text.split("\n")[0])
		for i in self.hunks:
			if needle == diff2filename(i.text.split("\n")[0]):
				return i.picked

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

def get_diff(files = ""):
	sock = os.popen("git diff HEAD %s" % files)
	lines = sock.readlines()
	sock.close()
	if len(lines) and lines[0].startswith("[1m"):
		print """It seems that you force using colors in your diffs
which is not compatible with darcs-git. Please set that value
to false or auto. Example:

git-config diff.color auto"""
		sys.exit(0)
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
  -s            --skip-long-comment      Don't give a long comment
  -h            --help                   shows brief description of command and its arguments"""
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.name = None
			self.all = None
			self.edit = None
			self.help = False
			self.files = ""
	options = Options()

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
	if emptydir(os.path.join(root, "refs", "heads")):
		first = True
		sock = os.popen("git status")
		lines = sock.readlines()
		sock.close()
		changes = True
		for i in lines:
			if i.startswith("nothing"):
				changes = False
		if changes:
			print "This is a new repo, can't cherry-pick for the first commit."
		else:
			print "No changes!"
			sys.exit(0)
	if first:
		status = Files([])
	else:
		# we need the overall status too, to exclude new files if necessary
		allstatus = scan_dir()
		status = scan_dir(options.files)
		if not options.all:
			status.hunks = askhunks(status.hunks)
	if first or status.hunks:
		if not options.name:
			options.name = ask("What is the patch name?", str)
	else:
		print "Ok, if you don't want to record anything, that's fine!"
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
			print "Invalid response, try again!"
	# in darcs, it was possible to simply rm a file and then record a
	# patch. support this
	os.system("git ls-files -z --deleted | git update-index -z --remove --stdin")
	if first or options.all:
		os.system("git commit -a -m '%s' %s %s" % (options.name, options.edit, options.files))
		sys.exit(0)
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
	for i in allstatus.hunks:
		if not status.ispicked(i):
			lines = i.text.split("\n")
			if "--- /dev/null" in lines:
				newlist.append(diff2filename(lines[0]))
	for i in newlist:
		os.system("git reset HEAD %s" % i)
	os.system("git commit -m '%s' %s" % (options.name, options.edit))
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

def changes(argv):
	def usage(ret):
		print """Usage: darcs-git changes [OPTION]... [FILE or DIRECTORY]...
Gives a changelog-style summary of the branch history.

Options:
  -l         --last=NUMBER         select the last NUMBER patches
  -s         --summary             summarize changes
  -v         --verbose             give verbose output
  -t         --tags                include tags in the log (darcs-git only)
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)

	class Options:
		def __init__(self):
			self.last = ""
			self.logopts = ""
			self.help = False
			self.tags = ""
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
			options.logopts = "--name-status"
		elif opt in ("-v", "--verbose"):
			options.logopts = "-p"
		elif opt in ("-t", "--tags"):
			options.tags = "| git name-rev --tags --stdin"
		elif opt in ("-h", "--help"):
			options.help = True
		optind += 1
	if optind < len(argv):
		options.files = " ".join(argv[optind:])
	if options.help:
		usage(0)
	return os.system("git log -M %s %s %s %s" % (options.last, options.logopts, options.files, options.tags))

def push(argv):
	def usage(ret):
		print """Usage: darcs-git push [OPTION]... [GIT OPTIONS]...
Copy and apply patches from this repository to another one.

Options:
  -a         --all                 answer yes to all questions
  -h         --help                shows brief description of command and its arguments"""
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
	if optind < len(argv):
		options.gitopts = " ".join(argv[optind:])
	if options.help:
		usage(0)
	sock = os.popen("git log origin/master..master --no-merges 2>&1")
	lines = sock.readlines()
	ret = sock.close()
	if not len(lines):
		print "No recorded local changes to push!"
		return
	print "".join(lines)
	if not options.all:
		while True:
			ret = ask("Do you want to push these patches? [ynq]")
			if ret == "y":
				break
			if ret in ("n", "q"):
				sys.exit(0)
			print "Invalid response, try again!"
	os.system("git push %s" % options.gitopts)

def pull(argv):
	def usage(ret):
		print """Usage: darcs-git pull [OPTION]... [GIT OPTIONS]...
Copy and apply patches to this repository from another one.

Options:
  -a         --all                 answer yes to all questions
  -h         --help                shows brief description of command and its arguments"""
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
	if optind < len(argv):
		options.gitopts = " ".join(argv[optind:])
	if options.help:
		usage(0)
	os.system("git fetch")
	sock = os.popen("git log master..origin/master --no-merges 2>&1")
	lines = sock.readlines()
	ret = sock.close()
	if not len(lines):
		print "No remote changes to pull!"
		return
	print "".join(lines)
	if not options.all:
		while True:
			ret = ask("Do you want to pull these patches? [ynq]")
			if ret == "y":
				break
			if ret in ("n", "q"):
				sys.exit(0)
			print "Invalid response, try again!"
	os.system("git pull %s" % options.gitopts)

def get(argv):
	def usage(ret):
		print """Usage: darcs-git get [OPTION]... <REPOSITORY> [<DIRECTORY>]
Create a local copy of another repository.
Use "darcs-git help clone" for more information.

Options:
  -h  --help                         shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	return os.system("git clone %s" % " ".join(argv))

def rollback(argv):
	def usage(ret):
		print """Usage: darcs-git rollback [OPTION]... <COMMIT-HASH>
Commit an inverse patch.
Use "darcs-git help revert" for more information.

Options:
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	return os.system("git revert %s" % " ".join(argv))

def unrecord(argv):
	def usage(ret):
		print """Usage: darcs-git unrecord [OPTION]...
Remove last committed patch without changing the working directory.
This is an alias for "git reset --soft HEAD^".

Options:
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	while True:
		ret = ask("Do you want to unrecord the last committed patch? [ynq]")
		if ret == "y":
			break
		if ret in ("n", "q"):
			sys.exit(0)
		print "Invalid response, try again!"
	os.system("git reset --soft HEAD^ %s" % " ".join(argv))
	print "Finished unrecording."

def unpull(argv):
	def usage(ret):
		print """Usage: darcs-git unpull [OPTION]...
Opposite of pull; unsafe if the latest patch is not in remote repository.
This is an alias for "git reset --hard HEAD^".

Options:
  -h         --help                shows brief description of command and its arguments"""
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
		print "Invalid response, try again!"
	os.system("git reset --hard HEAD^ %s" % " ".join(argv))
	print "Finished unpulling."

def optimize(argv):
	def usage(ret):
		print """Usage: darcs-git optimize [OPTION]...
Optimize the repository.
This is an alias for "git gc".

Options:
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	print "Checking how much disk space is wasted..."
	os.system("git count-objects")
	print "Chleaning up..."
	os.system("git gc")

def check(argv):
	def usage(ret):
		print """Usage: darcs-git check [OPTION]...
Check the repository for consistency.
This is an alias for "git fsck".

Options:
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	os.system("git fsck")

def trackdown(argv):
	def usage(ret):
		print """Usage: darcs-git trackdown [OPTION]...
Locate the most recent version lacking an error.
This is an alias for "git bisect".

Options:
  -h         --help                shows brief description of command and its arguments"""
		sys.exit(ret)
	if len(argv) and argv[0] in ("-h", "--help"):
		usage(0)
	os.system("git bisect")

def main(argv):
	def usage(ret):
		print """Usage: darcs-git COMMAND ...

A darcs-like interface for git with ~22 commands. (Git has ~144.)

The meaning of the letters are the following:
  A             Alias.
                Example: darcs-git get does exactly the same as git clone.
  Y             Yes, supported, and dg adds some extra features.
                Example: darcs-git push tries to find out what will be pushed.
  W             Supported unintentionally, because the darcs-git wrapper calls
                  git <subcommand> for all unknown commands.
		Example: darcs-git mv
  N             Not supported.
                Example: darcs-git put is not supported.

Commands:
  W help          Display help for darcs or a single commands.
Changing and querying the working copy:
  W add           Add one or more new files or directories.
  W remove        Remove one or more files or directories from the repository.
  W mv            Move/rename one or more files or directories.
  N replace       Replace a token with a new value for that token.
  Y revert        Revert to the recorded version (safe the first time only).
  N unrevert      Undo the last revert (may fail if changes after the revert).
  Y whatsnew      Display unrecorded changes in the working copy.
Copying changes between the working copy and the repository:
  Y record        Save changes in the working copy to the repository as a patch.
  A unrecord      Remove recorded patches without changing the working copy.
  N amend-record  Replace a patch with a better version before it leaves your repository.
  N resolve       Mark any conflicts to the working copy for manual resolution.
Direct modification of the repository:
  W tag           Tag the contents of the repository with a version name.
  N setpref       Set a value for a preference (test, predist, ...).
  A rollback      Record an inverse patch without changing the working directory.
Querying the repository:
  W diff          Create a diff between two versions of the repository.
  Y changes       Gives a changelog-style summary of the repository history.
  W annotate      Display which patch last modified something.
  N dist          Create a distribution tarball.
  A trackdown     Locate the most recent version lacking an error.
  N query         Query information which is stored by darcs.
Copying patches between repositories with working copy update:
  Y pull          Copy and apply patches from another repository to this one.
  A unpull        Opposite of pull; unsafe if patch is not in remote repository.
  N obliterate    Delete selected patches from the repository. (UNSAFE!)
  Y push          Copy and apply patches from this repository to another one.
  N send          Send by email a bundle of one or more patches.
  W apply         Apply patches (from an email bundle) to the repository.
  A get           Create a local copy of another repository.
  N put           Makes a copy of the repository
Administrating repositories:
  W initialize    Initialize a new source tree as a darcs repository.
  A optimize      Optimize the repository.
  A check         Check the repository for consistency.
  N repair        Repair the corrupted repository.
"""
		sys.exit(ret)
	if len(sys.argv) == 1 or sys.argv[1] == "-h":
		usage(0)
	else:
		# this will exit if no root found
		if sys.argv[1] not in ["init", "get"]:
			get_root()
		os.environ['GIT_PAGER'] = 'cat'
		if sys.argv[1][:3] == "rec":
			record(argv[1:])
		elif sys.argv[1][:3] == "rev":
			revert(argv[1:])
		elif sys.argv[1][:4] == "what":
			whatsnew(argv[1:])
		elif sys.argv[1][:4] == "chan":
			changes(argv[1:])
		elif sys.argv[1] == "push":
			push(argv[1:])
		elif sys.argv[1] == "pull":
			pull(argv[1:])
		elif sys.argv[1] == "get":
			get(argv[1:])
		elif sys.argv[1][:4] == "roll":
			rollback(argv[1:])
		elif sys.argv[1][:5] == "unrec":
			unrecord(argv[1:])
		elif sys.argv[1] == "unpull":
			unpull(argv[1:])
		elif sys.argv[1][:3] == "opt":
			optimize(argv[1:])
		elif sys.argv[1] == "check":
			check(argv[1:])
		elif sys.argv[1][:5] == "track":
			trackdown(argv[1:])
		else:
			os.system("git %s" % " ".join(argv))

if __name__ == "__main__":
	main(sys.argv[1:])
