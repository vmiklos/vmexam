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

import sys
import tty
import termios
import os
import getopt
import subprocess


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


def record(argv):
    s = subprocess.run(["git", "diff", "--quiet", "HEAD"])
    if s.returncode == 0:
        print("Ok, if you don't want to record anything, that's fine!")
        sys.exit(0)
    add = ["git", "add", "--patch"]
    add.extend(argv)
    subprocess.run(add, check=True)
    message = ask("What is the commit message?", str)
    while True:
        ret = ask("Do you want to add a long comment? [ynq]")
        if ret == "q":
            sys.exit(0)
        if ret in ("y", "n"):
            edit = ret == "y"
            break
        print("Invalid response, try again!")
    commit = ["git", "commit", "-m", message]
    if edit:
        commit.append("-e")
    subprocess.run(commit, check=True)


def revert(argv):
    s = subprocess.run(["git", "diff", "--quiet", "HEAD"])
    if s.returncode == 0:
        print("Ok, if you don't want to revert anything, that's fine!")
        sys.exit(0)
    checkout = ["git", "checkout", "--patch"]
    checkout.extend(argv)
    subprocess.run(checkout, check=True)


def whatsnew(argv):
    diff = ["git", "diff", "HEAD", "-M", "-C", "--exit-code"]
    opts, args = getopt.getopt(argv, "s", ["summary"])
    optind = 0
    for opt, arg in opts:
        if opt in ("-s", "--summary"):
            diff.append("--name-status")
        optind += 1
    if optind < len(argv):
        diff.extend(argv[optind:])
    s = subprocess.run(diff)
    if s.returncode == 0:
        print("No changes!")


def push(argv):
    output = subprocess.check_output(["git", "log", "HEAD@{upstream}.."])
    if not len(output):
        print("No recorded local changes to push!")
        return 0
    print(output.decode("utf-8"))
    while True:
        ret = ask("Do you want to push these patches? [ynq]")
        if ret == "y":
            break
        if ret in ("n", "q"):
            sys.exit(0)
        print("Invalid response, try again!")
    ret = os.system("git push")
    if ret:
        ret = pull(['-a'])
        if not ret:
            ret = os.system("git push")
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
    os.system("git fetch %s" % options.gitopts)
    remote = "%s/%s" % (options.gitopts, get_merge(branch))
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
    if os.system("git pull --rebase %s" % options.gitopts) != 0:
        return(1)
    if changes and os.system("git stash pop") != 0:
        return(1)


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


def main(argv):
    import time
    date_prefix = time.strftime("%Y-%m-%d %H:%M:%S")
    with open(os.path.expanduser("~/.local/state/darcs-git/commands.log"), "a") as stream:
        stream.write("{} {}\n".format(date_prefix, argv))

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
        os.environ['GIT_PAGER'] = 'cat'
        if sys.argv[1][:3] == "rec":
            return record(argv[1:])
        elif sys.argv[1][:3] == "rev":
            return revert(argv[1:])
        elif sys.argv[1][:4] == "what":
            return whatsnew(argv[1:])
        elif sys.argv[1] == "push":
            return push(argv[1:])
        elif sys.argv[1] == "pull":
            return pull(argv[1:])
        elif sys.argv[1][:5] == "unrec":
            return unrecord(argv[1:])
        elif sys.argv[1] == "unpull":
            return unpull(argv[1:])
        else:
            return os.system("git '%s'" % "' '".join(argv))


if __name__ == "__main__":
    if main(sys.argv[1:]) != 0:
        sys.exit(1)
    else:
        sys.exit(0)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
