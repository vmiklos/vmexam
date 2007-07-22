import os, re, sys, pickle

records = {}

def parse_file(filename):
	global records

	sock = open(filename)
	lines = sock.readlines()
	sock.close()

	for i in lines:
		line = i.strip()
		if re.match(r'..:.. <.Mojojojo> [^ ]+, [^ ]+ => ', line):
			record = re.sub(r"..:.. <[^>]+> [^ ]+, ([^ ]+) => (.*)", "\\1\n\\2", line).split("\n")
			records[record[0].replace("[WFA]", "")] = record[1]

sock = open(sys.argv[1], "w")

flist = []
for root, dirs, files in os.walk(os.environ['HOME'] + "/.irssi/logs/freenode"):
	for file in files:
		if file.startswith("#debian.hu_"):
			flist.append(os.path.join(root, file))
flist.sort()
for i in flist:
	parse_file(i)

print "processed %d records" % len(records.keys())
pickle.dump(records, sock)
sock.close()
print "saved to %s" % sys.argv[1]
