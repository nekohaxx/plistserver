import http.server
import ssl
import os
PORT = 4443
DIRECTORY = "."
os.chdir(DIRECTORY)
handler = http.server.SimpleHTTPRequestHandler
httpd = http.server.HTTPServer(('0.0.0.0', PORT), handler)
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(certfile='192.168.12.158.pem', keyfile='192.168.12.158-key.pem')
httpd.socket = context.wrap_socket(httpd.socket, server_side=True)
httpd.serve_forever()

