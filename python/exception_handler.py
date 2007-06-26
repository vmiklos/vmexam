import traceback, inspect, sys

def foo():
	1/0

def on_bug():
	# Python style
	type, value, tb = sys.exc_info()
	stype = str(type).split("'")[1]
	print "Traceback (most recent call last):"
	print "".join(traceback.format_tb(tb)).strip()
	print "%s: %s" % (stype, value)

	# ~ C style
	badline = inspect.trace()[-1]
	print "%s at file %s line %d" % (stype, badline[1], badline[2])

print "foo"
try:
	foo()
except Exception:
	on_bug()
print "bar"
