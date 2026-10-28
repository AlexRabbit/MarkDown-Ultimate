window.mduStartTour = function (stepsJson) {
  if (!window.driver || !window.driver.js || !window.driver.js.driver) {
    console.warn("Driver.js not loaded");
    return;
  }
  const steps = JSON.parse(stepsJson);
  const driver = window.driver.js.driver;
  const drv = driver({
    showProgress: true,
    animate: true,
    allowClose: true,
    overlayOpacity: 0.45,
    popoverClass: "mdu-driver-popover",
    steps: steps.map(function (s) {
      return {
        element: s.element,
        popover: {
          title: s.title,
          description: s.description,
          side: s.side || "bottom",
          align: s.align || "start",
        },
      };
    }),
  });
  drv.drive();
};

window.mduRenderMath = function (rootSelector) {
  if (!window.renderMathInElement) return;
  var el = document.querySelector(rootSelector);
  if (!el) return;
  window.renderMathInElement(el, {
    delimiters: [
      { left: "$$", right: "$$", display: true },
      { left: "$", right: "$", display: false },
    ],
    throwOnError: false,
  });
};

window.mduApplyEditorCommand = function (editorId, before, after, placeholder) {
  var el = document.getElementById(editorId);
  if (!el || el.tagName !== "TEXTAREA") return null;
  var start = el.selectionStart;
  var end = el.selectionEnd;
  var text = el.value;
  var selected = text.slice(start, end) || placeholder || "";
  var next = text.slice(0, start) + before + selected + after + text.slice(end);
  el.value = next;
  var cursor = start + before.length + selected.length;
  el.selectionStart = el.selectionEnd = cursor;
  el.focus();
  el.dispatchEvent(new Event("input", { bubbles: true }));
  return el.value;
};

window.mduReadEditor = function (editorId) {
  var el = document.getElementById(editorId);
  if (!el || el.tagName !== "TEXTAREA") return null;
  return el.value;
};

window.mduPrintMarkdown = function (title, html) {
  var w = window.open("", "_blank");
  if (!w) {
    alert("Allow pop-ups to print or save as PDF.");
    return false;
  }
  w.document.open();
  w.document.write(
    "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>" +
      (title || "Document").replace(/</g, "&lt;") +
      "</title><style>body{font-family:Segoe UI,system-ui,sans-serif;padding:2rem;max-width:800px;margin:0 auto;line-height:1.5} table{border-collapse:collapse;width:100%} th,td{border:1px solid #ccc;padding:.35rem .5rem}</style></head><body>" +
      html +
      "</body></html>"
  );
  w.document.close();
  setTimeout(function () {
    try {
      w.focus();
      w.print();
    } catch (e) {
      console.warn("print failed", e);
    }
  }, 300);
  return true;
};

window.mduCopyText = function (text) {
  if (!text) return false;
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(text).catch(function () {});
    return true;
  }
  try {
    var ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.left = "-9999px";
    document.body.appendChild(ta);
    ta.select();
    var ok = document.execCommand("copy");
    document.body.removeChild(ta);
    return ok;
  } catch (e) {
    return false;
  }
};

window.mduInitToolsSidebarResize = function (handleId, storageKey) {
  var handle = document.getElementById(handleId);
  var panel = document.getElementById("toolsSidebar");
  if (!handle || !panel || handle._mduResizeBound) return;
  handle._mduResizeBound = true;
  var dragging = false;
  handle.addEventListener("mousedown", function (ev) {
    dragging = true;
    ev.preventDefault();
    document.body.classList.add("mdu-resizing");
  });
  window.addEventListener("mousemove", function (ev) {
    if (!dragging) return;
    var w = window.innerWidth - ev.clientX;
    w = Math.max(240, Math.min(Math.floor(window.innerWidth * 0.5), w));
    panel.style.width = w + "px";
    try {
      localStorage.setItem(storageKey, String(w));
    } catch (e) {}
  });
  window.addEventListener("mouseup", function () {
    if (!dragging) return;
    dragging = false;
    document.body.classList.remove("mdu-resizing");
    var w = parseInt(panel.style.width, 10);
    if (!isNaN(w)) {
      try {
        localStorage.setItem(storageKey, String(w));
      } catch (e) {}
      window.dispatchEvent(new CustomEvent("mdu-tools-width", { detail: w }));
    }
  });
};

window.mduJumpToOffset = function (editorId, offset) {
  var el = document.getElementById(editorId);
  if (!el || el.tagName !== "TEXTAREA") return;
  var o = Math.max(0, Math.min(el.value.length, offset));
  el.focus();
  el.selectionStart = el.selectionEnd = o;
};

function mduLineAt(text, pos) {
  var start = text.lastIndexOf("\n", pos - 1) + 1;
  var end = text.indexOf("\n", pos);
  if (end === -1) end = text.length;
  return { start: start, end: end, line: text.slice(start, end) };
}

function mduIsTableLine(line) {
  return /^\s*\|(.+)\|\s*$/.test(line);
}

window.mduJsEditorAction = function (editorId, action) {
  var el = document.getElementById(editorId);
  if (!el || el.tagName !== "TEXTAREA") return null;
  var text = el.value;
  var pos = el.selectionStart;
  var block = mduLineAt(text, pos);
  if (!mduIsTableLine(block.line)) {
    return el.value;
  }
  var lines = text.split("\n");
  var lineIdx = text.slice(0, block.start).split("\n").length - 1;
  var cols = block.line.split("|").length - 2;
  if (cols < 1) cols = 2;
  var emptyRow = "|" + "  |".repeat(cols);
  var sep = "|" + " --- |".repeat(cols);
  if (action === "table-row-below") {
    lines.splice(lineIdx + 1, 0, emptyRow);
  } else if (action === "table-row-above") {
    lines.splice(lineIdx, 0, emptyRow);
  } else if (action === "table-col-right" || action === "table-col-left") {
    for (var i = 0; i < lines.length; i++) {
      if (!mduIsTableLine(lines[i])) continue;
      if (/^\s*\|[\s\-:|]+\|\s*$/.test(lines[i])) {
        lines[i] = sep;
      } else {
        if (action === "table-col-right") {
          lines[i] = lines[i].replace(/\|\s*$/, "  |");
        } else {
          lines[i] = lines[i].replace(/^\s*\|/, "|  ");
        }
      }
    }
  }
  el.value = lines.join("\n");
  el.dispatchEvent(new Event("input", { bubbles: true }));
  return el.value;
};

window.mduFindReplace = function (editorId, find, replace, replaceAll) {
  var el = document.getElementById(editorId);
  if (!el || el.tagName !== "TEXTAREA" || !find) return el ? el.value : null;
  if (replaceAll) {
    el.value = el.value.split(find).join(replace);
  } else {
    var idx = el.value.indexOf(find, el.selectionStart);
    if (idx === -1) idx = el.value.indexOf(find);
    if (idx !== -1) {
      el.value = el.value.slice(0, idx) + replace + el.value.slice(idx + find.length);
      el.selectionStart = idx;
      el.selectionEnd = idx + replace.length;
    }
  }
  el.focus();
  el.dispatchEvent(new Event("input", { bubbles: true }));
  return el.value;
};

window.mduInitAppShortcuts = function () {
  if (window._mduShortcutsBound) return;
  window._mduShortcutsBound = true;
  document.addEventListener("keydown", function (ev) {
    if (!ev.ctrlKey) return;
    var k = ev.key.toLowerCase();
    if (k === "f") {
      ev.preventDefault();
      window.dispatchEvent(new CustomEvent("mdu-open-modal", { detail: "find-replace" }));
    } else if (k === "/") {
      ev.preventDefault();
      window.dispatchEvent(new CustomEvent("mdu-open-modal", { detail: "shortcuts" }));
    } else if (k === "b") {
      ev.preventDefault();
      window.dispatchEvent(new CustomEvent("mdu-run-command", { detail: "bold" }));
    } else if (k === "i") {
      ev.preventDefault();
      window.dispatchEvent(new CustomEvent("mdu-run-command", { detail: "italic" }));
    }
  });
};
