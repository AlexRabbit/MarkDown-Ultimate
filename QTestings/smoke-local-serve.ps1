$ErrorActionPreference = "Stop"
$port = if ($env:MDU_PORT) { $env:MDU_PORT } else { "8765" }
python -c @"
import urllib.request
base = 'http://127.0.0.1:$port/MarkDown-Ultimate'
r = urllib.request.urlopen(base + '/')
html = r.read().decode('utf-8', 'replace')
assert 'MarkDown Ultimate' in html
import re
m = re.search(r'markdown_ultimate_frontend-[a-f0-9]+_bg\.wasm', html)
assert m, 'wasm href missing'
wasm = urllib.request.urlopen(base + '/' + m.group(0))
assert len(wasm.read()) > 100000
js = urllib.request.urlopen(base + '/js/mdu-ui.js')
assert js.status == 200
print('Local serve smoke OK')
"@
