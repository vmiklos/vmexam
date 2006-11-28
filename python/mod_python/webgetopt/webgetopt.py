from mod_python import apache, util

class webgetopt:
	"""
	A simple QUERY_STRING / PATH_INFO parser.
	When creating an instance of the class, you must pass the Apache
	request handler.
	After that you can reach the queryString and pathInfo dictionaries and
	the pathInfoExtra attribute if there is an extra variable at the end of
	PATH_INFO.
	"""
	def __getPathInfo(self, list):
		return dict([[list[i], list[i+1]] for i in range(len(list)) if i%2 == 0])
	def __init__(self, req):
		self.queryString = {}
		self.pathInfo = {}
		self.pathInfoExtra = ""

		form = util.FieldStorage(req)
		for i in form.keys():
			self.queryString[i] = form[i]
		try:
			list = req.subprocess_env['PATH_INFO'].strip('/').split('/')
			try:
				self.pathInfo = self.__getPathInfo(list)
			except IndexError:
				self.pathInfoExtra = list[-1]
				list.remove(list[-1])
				self.pathInfo = self.__getPathInfo(list)
		except KeyError:
			pass

if __name__ == "webgetopt":
	def handler(req):
		req.content_type = 'text/html'
		opts = webgetopt(req)
		if len(opts.queryString):
			req.write("opts.queryString: <br />")
			req.write(str(opts.queryString))
			req.write("<br />")
		if len(opts.pathInfo):
			req.write("opts.pathInfo: <br />")
			req.write(str(opts.pathInfo))
			req.write("<br />")
		if len(opts.pathInfoExtra):
			req.write("opts.pathInfoExtra: <br />")
			req.write(opts.pathInfoExtra)
			req.write("<br />")
		return apache.OK
