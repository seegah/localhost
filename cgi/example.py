#!/usr/bin/python3
import sys
import os

print("Content-Type: text/html\r\n\r\n")
print("<html><body>")
print(f"<h1>Hello from Python CGI!</h1>")
print(f"<p>Path: {os.environ.get('PATH_INFO', 'N/A')}</p>")
print("</body></html>")