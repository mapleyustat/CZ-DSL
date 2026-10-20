#!/usr/bin/env python3
"""
简单的HTTP服务器用于测试CZ-DSL用户界面
"""

import http.server
import socketserver
import os
import sys
from pathlib import Path
import mimetypes

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # 添加CORS头部以支持本地开发
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()
    
    def guess_type(self, path):
        # 确保正确的MIME类型
        mimetype, _ = mimetypes.guess_type(path)
        if path.endswith('.js'):
            return 'application/javascript'
        elif path.endswith('.css'):
            return 'text/css'
        elif path.endswith('.html'):
            return 'text/html'
        return mimetype or 'application/octet-stream'

def main():
    # 切换到frontend目录
    frontend_dir = Path(__file__).parent
    os.chdir(frontend_dir)
    
    PORT = 8080
    
    print(f"🌊 CZ-DSL 用户界面测试服务器")
    print(f"📁 服务目录: {frontend_dir}")
    print(f"🌐 访问地址: http://localhost:{PORT}/user_interface.html")
    print(f"⚡ 按 Ctrl+C 停止服务器")
    print("-" * 50)
    
    try:
        with socketserver.TCPServer(("", PORT), CustomHTTPRequestHandler) as httpd:
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 服务器已停止")
        sys.exit(0)
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"❌ 端口 {PORT} 已被占用，请尝试其他端口")
            sys.exit(1)
        else:
            raise

if __name__ == "__main__":
    main()
