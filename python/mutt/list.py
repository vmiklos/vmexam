from imaplib import *
import sys

"""
this is a small python script to list your imap folders in mutt
usage:

	mailboxes `python ~/.mutt/list.py mail "ssh -q foo.com /usr/sbin/imapd"`

	or

	mailboxes `python ~/.mutt/list.py bar.com nick@bar.com secret ""`
"""

class ImapLister:
	def __init__(self, server = None, user = None, pwd = None, dir = None,
			tunnel = None):
		self.dirs = []

		if not tunnel:
			sock = IMAP4_SSL(server)
			sock.login(user, pwd)
		else:
			sock = IMAP4_stream(tunnel)
		for i in sock.list(dir)[1]:
			if i.split(" ")[-1] == dir:
				i = "INBOX"
			self.dirs.append(i.split(" ")[-1])
		self.dirs.sort()

if __name__ == "__main__":
	if len(sys.argv) == 3:
		server = ImapLister(dir = sys.argv[1], tunnel = sys.argv[2])
	else:
		# we have two more parameter for real imap servers
		server = ImapLister(server = sys.argv[1], user = sys.argv[2],
				pwd = sys.argv[3], dir = sys.argv[4])
	for i in server.dirs:
		sys.stdout.write("=%s " % i)
