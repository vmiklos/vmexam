import irssi, cStringIO, operator, time

# provides a /iinfo command to show your idle times on different channels

# From: http://aspn.activestate.com/ASPN/Cookbook/Python/Recipe/267662
def indent(rows, hasHeader=False, headerChar='-', delim='  ', justify='left',
		separateRows=False, prefix='', postfix='', wrapfunc=lambda x:x):
	"""Indents a table by column.
	- rows: A sequence of sequences of items, one sequence per row.
	- hasHeader: True if the first row consists of the columns' names.
	- headerChar: Character to be used for the row separator line
		 (if hasHeader==True or separateRows==True).
	- delim: The column delimiter.
	- justify: Determines how are data justified in their column. 
		 Valid values are 'left','right' and 'center'.
	- separateRows: True if rows are to be separated by a line
		 of 'headerChar's.
	- prefix: A string prepended to each printed row.
	- postfix: A string appended to each printed row.
	- wrapfunc: A function f(text) for wrapping text; each element in
		 the table is first wrapped by this function."""
	# closure for breaking logical rows to physical, using wrapfunc
	def rowWrapper(row):
		newRows = [wrapfunc(item).split('\n') for item in row]
		return [[substr or '' for substr in item] for item in map(None,*newRows)]
	# break each logical row into one or more physical ones
	logicalRows = [rowWrapper(row) for row in rows]
	# columns of physical rows
	columns = map(None,*reduce(operator.add,logicalRows))
	# get the maximum of each column by the string length of its items
	maxWidths = [max([len(str(item)) for item in column]) for column in columns]
	rowSeparator = headerChar * (len(prefix) + len(postfix) + sum(maxWidths) + \
								 len(delim)*(len(maxWidths)-1))
	# select the appropriate justify method
	justify = {'center':str.center, 'right':str.rjust, 'left':str.ljust}[justify.lower()]
	output=cStringIO.StringIO()
	if separateRows: print >> output, rowSeparator
	for physicalRows in logicalRows:
		for row in physicalRows:
			print >> output, \
				prefix \
				+ delim.join([justify(str(item),width) for (item,width) in zip(row,maxWidths)]) \
				+ postfix
		if separateRows or hasHeader: print >> output, rowSeparator; hasHeader=False
	return output.getvalue()

def how_old(epoch):
	age = int(time.time()) - int(epoch)
	if age > 60*60*24*365*2:
		s = str(age/60/60/24/365)
		s += " years"
	elif age > 60*60*24*(365/12)*2:
		s = str(age/60/60/24/(365/12))
		s += " months"
	elif age > 60*60*24*7*2:
		s = str(age/60/60/24/7)
		s += " weeks"
	elif age > 60*60*24*2:
		s = str(age/60/60/24)
		s += " days"
	elif age > 60*60*2:
		s = str(age/60/60)
		s += " hours"
	elif age > 60*2:
		s = str(age/60)
		s += " minutes"
	else:
		s = str(age)
		s += " seconds"
	return s

idles = {}

def cmd_iinfo(data, server, witem):
	global idles
	labels = ('Server', 'Channel', 'Idle')
	servers = []
	for i in irssi.servers():
		for j in i.channels():
			if i.tag in idles.keys() and j.name in idles[i.tag].keys():
				idle = how_old(idles[i.tag][j.name])
			else:
				if not i.tag in idles.keys():
					idles[i.tag] = {}
				if j.name not in idles[i.tag].keys():
					idles[i.tag][j.name] = time.time()
				idle = "n/a"
			servers.append([i.tag, j.name, idle])
	print
	print indent([labels]+servers).strip()
	print

def send(msg, server, witem):
	global idles
	if not server or not witem:
		return
	if server.tag not in idles.keys():
		idles[server.tag] = {witem.name: time.time()}
	else:
		idles[server.tag][witem.name] = time.time()

irssi.command_bind('iinfo', cmd_iinfo)
irssi.signal_add("send command", send)
