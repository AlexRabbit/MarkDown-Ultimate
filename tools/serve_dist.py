#!/usr/bin/env python3
"""Serve dist/ so /MarkDown-Ultimate/* URLs work locally (same as GitHub Pages)."""
from __future__ import annotations

import mimetypes
import os
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parent.parent / "dist"
PREFIX = "/MarkDown-Ultimate"
PORT = int(os.environ.get("MDU_PORT", "8765"))


class Handler(BaseHTTPRequestHandler):
    def log_message(self, fmt: str, *args) -> None:
        sys.stderr.write(f"[mdu-serve] {fmt % args}\n")

    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        path = unquote(parsed.path)

        if path in ("", "/"):
            self.send_response(302)
            self.send_header("Location", PREFIX + "/")
            self.end_headers()
            return

        if path == PREFIX:
            self.send_response(302)
            self.send_header("Location", PREFIX + "/")
            self.end_headers()
            return

        if path == PREFIX + "/" or path == PREFIX + "/index.html":
            rel = "index.html"
        elif path.startswith(PREFIX + "/"):
            rel = path[len(PREFIX) + 1 :]
        else:
            self.send_error(404, "Use /MarkDown-Ultimate/")
            return

        if rel.endswith("/"):
            rel += "index.html"

        file_path = (ROOT / rel).resolve()
        if not str(file_path).startswith(str(ROOT.resolve())):
            self.send_error(403)
            return
        if not file_path.is_file():
            self.send_error(404, f"Not found: {rel}")
            return

        ctype, _ = mimetypes.guess_type(str(file_path))
        if file_path.suffix == ".wasm":
            ctype = "application/wasm"
        size = file_path.stat().st_size
        self.send_response(200)
        self.send_header("Content-Type", ctype or "application/octet-stream")
        self.send_header("Content-Length", str(size))
        self.end_headers()
        try:
            with open(file_path, "rb") as f:
                while True:
                    chunk = f.read(1024 * 256)
                    if not chunk:
                        break
                    self.wfile.write(chunk)
        except (BrokenPipeError, ConnectionResetError):
            pass


def main() -> None:
    if not (ROOT / "index.html").is_file():
        sys.stderr.write(f"Missing {ROOT / 'index.html'} — run: cd frontend && trunk build --release\n")
        sys.exit(1)
    host = "127.0.0.1"
    httpd = HTTPServer((host, PORT), Handler)
    url = f"http://{host}:{PORT}{PREFIX}/"
    sys.stderr.write(f"Serving {ROOT} at {url}\n")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
