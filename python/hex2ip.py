import socket, sys

def hex2ip(s):
	return ".".join(["%d"%int(n, 16) for n in (s[0:2],s[2:4],s[4:6],s[6:8])])
try:
	host = socket.gethostbyaddr(hex2ip(sys.argv[1]))
except socket.error, str:
	print str[1]
	sys.exit(-1)
print host[0]
