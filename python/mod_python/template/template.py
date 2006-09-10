from mod_python import apache, psp

def handler(req):
	req.content_type = 'text/html'
	template = psp.PSP(req, filename='template.html')
	template.run({'what':'world'})
	return apache.OK
