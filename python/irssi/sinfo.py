import irssi, cStringIO, operator

"""
a replacement for /server list
- i always typed /server info and got disconnected..
- it prints nicks
- it prints disconnected servers
"""

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

def cmd_sinfo(data, server, witem):
	labels = ('Server', 'Port', 'Network', 'Settings', 'Status', 'Nick')
	online = []
	servers = []
	for i in irssi.servers():
		if i.connect.use_ssl:
			ssl = "ssl"
		else:
			ssl = ""
		servers.append([i.connect.address, str(i.connect.port), i.tag, ssl, "connected", i.nick])
		online.append(i.tag)
	for i in irssi.chatnets():
		if i.name not in online:
			servers.append(["", "", i.name, "", "disconnected", ""])
	print indent([labels]+servers).strip()

irssi.command_bind('sinfo', cmd_sinfo)
