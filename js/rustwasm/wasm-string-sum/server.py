#!/usr/bin/python3

import http.server
import socketserver

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map[".wasm"] = "application/wasm"

with socketserver.TCPServer(("", 8000), Handler) as httpd:
    print("Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...")
    httpd.serve_forever()
