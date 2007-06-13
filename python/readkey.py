import sys, tty, termios

def getc():
	fd = sys.stdin.fileno()
	old_settings = termios.tcgetattr(fd)
	try:
		tty.setraw(sys.stdin.fileno())
		c = sys.stdin.read(1)
	finally:
		termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
	print c
	return c

sys.stdout.write("yes or no? [y/n]")
s = getc()
print "your answer was %s" % s
