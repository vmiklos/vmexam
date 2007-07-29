import xmlrpclib
testsvr = xmlrpclib.Server("http://localhost:1873")

print testsvr.inc(1)
