from SimpleXMLRPCServer import SimpleXMLRPCServer

class Actions:
	def inc(self, x):
		return x + 1

if __name__ == "__main__":
	server = SimpleXMLRPCServer(('',1873))
	server.register_instance(Actions())
	server.serve_forever()
