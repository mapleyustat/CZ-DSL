#!/usr/bin/env python3
"""
Simple HTTP server for CZ-DSL frontend
"""

import http.server
import socketserver
import os
import sys
from pathlib import Path

class CZDSLHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(Path(__file__).parent), **kwargs)
    
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()
    
    def do_GET(self):
        # Serve index.html for root path
        if self.path == '/':
            self.path = '/index.html'
        return super().do_GET()

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    
    with socketserver.TCPServer(("", port), CZDSLHandler) as httpd:
        print(f"🌊 CZ-DSL Frontend Server")
        print(f"📡 Serving at http://localhost:{port}")
        print(f"📁 Directory: {Path(__file__).parent}")
        print(f"🚀 Press Ctrl+C to stop")
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")

if __name__ == "__main__":
    main()