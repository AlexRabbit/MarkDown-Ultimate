#!/usr/bin/env python3
"""Serve dist/ so /MarkDown-Ultimate/* URLs work locally (same as GitHub Pages)."""
from __future__ import annotations

import mimetypes
import os
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parent.parent / "dist"
PREFIX = "/MarkDown-Ultimate"
PORT = int(os.environ.get("MDU_PORT", "8765"))
QUIET = os.environ.get("MDU_QUIET", "").strip() not in ("", "0", "false", "False")
LOG_PATH = Path(__file__).resolve().parent.parent / "logs" / "hot" / "serve.log"


class ReuseThreadingServer(ThreadingHTTPServer):
    allow_reuse_address = True
    daemon_threads = True


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, fmt: str, *args) -> None:
        line = f"[mdu-serve] {fmt % args}\n"
        if QUIET:
            try:
                LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
                with LOG_PATH.open("a", encoding="utf-8") as f:
                    f.write(line)
            except OSError:
                pass
            return
        sys.stderr.write(line)

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
        data = file_path.read_bytes()
        self.send_response(200)
        self.send_header("Content-Type", ctype or "application/octet-stream")
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-cache")
        self.end_headers()
        try:
            self.wfile.write(data)
        except (BrokenPipeError, ConnectionResetError):
            pass


def main() -> None:
    if not (ROOT / "index.html").is_file():
        sys.stderr.write(
            f"Missing {ROOT / 'index.html'} — run: cd frontend && trunk build --release\n"
        )
        sys.exit(1)
    host = "127.0.0.1"
    httpd = ReuseThreadingServer((host, PORT), Handler)
    url = f"http://{host}:{PORT}{PREFIX}/"
    banner = f"Serving {ROOT} at {url}\n"
    if QUIET:
        try:
            LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
            LOG_PATH.write_text(banner, encoding="utf-8")
        except OSError:
            pass
    else:
        sys.stderr.write(banner)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
