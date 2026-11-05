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
  var parts = (rootSelector || "").split(",");
  for (var p = 0; p < parts.length; p++) {
    var sel = parts[p].trim();
    if (!sel) continue;
    var el = document.querySelector(sel);
    if (!el) continue;
    window.renderMathInElement(el, {
      delimiters: [
        { left: "$$", right: "$$", display: true },
        { left: "$", right: "$", display: false },
      ],
      throwOnError: false,
    });
  }
  if (window.mduSyncPreviewLayout) window.mduSyncPreviewLayout();
};

function mduGetSourceTextarea() {
  return document.getElementById("mduSourceEditor");
}

function mduPreviewEditorActive() {
  var pe = document.getElementById("previewEditor");
  var sp = document.getElementById("previewPane");
  if (!pe && !sp) return false;
  var ae = document.activeElement;
  if (pe && (ae === pe || (ae && pe.contains(ae)))) return true;
  if (sp && sp.getAttribute("contenteditable") === "true" && (ae === sp || (ae && sp.contains(ae)))) return true;
  return false;
}

function mduActivePreviewEl() {
  var pe = document.getElementById("previewEditor");
  var sp = document.getElementById("previewPane");
  var ae = document.activeElement;
  if (pe && (ae === pe || (ae && pe.contains(ae)))) return pe;
  if (sp && (ae === sp || (ae && sp.contains(ae)))) return sp;
  return pe || sp;
}

function mduInsertAtTextarea(el, text, selectLen) {
  if (!el || el.tagName !== "TEXTAREA") return null;
  var start = el.selectionStart;
  var end = el.selectionEnd;
  el.value = el.value.slice(0, start) + text + el.value.slice(end);
  var cursor = start + text.length;
  if (typeof selectLen === "number") {
    el.selectionStart = start;
    el.selectionEnd = start + selectLen;
  } else {
    el.selectionStart = el.selectionEnd = cursor;
  }
  el.focus();
  el.dispatchEvent(new Event("input", { bubbles: true }));
  return el.value;
}

function mduWalkHtmlToMarkdown(node) {
  if (!node) return "";
  if (node.nodeType === 3) return node.textContent || "";
  if (node.nodeType !== 1) return "";
  var name = node.nodeName;
  if (name === "BR") return "\n";
  var inner = "";
  for (var i = 0; i < node.childNodes.length; i++) {
    inner += mduWalkHtmlToMarkdown(node.childNodes[i]);
  }
  if (name === "P" || name === "DIV") return inner.trim() + "\n\n";
  if (name === "H1") return "# " + node.textContent.trim() + "\n\n";
  if (name === "H2") return "## " + node.textContent.trim() + "\n\n";
  if (name === "H3") return "### " + node.textContent.trim() + "\n\n";
  if (name === "H4") return "#### " + node.textContent.trim() + "\n\n";
  if (name === "H5") return "##### " + node.textContent.trim() + "\n\n";
  if (name === "H6") return "###### " + node.textContent.trim() + "\n\n";
  if (name === "STRONG" || name === "B") return "**" + inner + "**";
  if (name === "EM" || name === "I") return "*" + inner + "*";
  if (name === "S" || name === "DEL" || name === "STRIKE") return "~~" + inner + "~~";
  if (name === "CODE") return "`" + inner + "`";
  if (name === "PRE") return "```\n" + (node.textContent || "").trim() + "\n```\n\n";
  if (name === "BLOCKQUOTE") {
    return inner
      .trim()
      .split("\n")
      .map(function (ln) {
        return "> " + ln;
      })
      .join("\n") + "\n\n";
  }
  if (name === "UL") {
    var u = "";
    node.querySelectorAll(":scope > li").forEach(function (li) {
      u += "- " + li.textContent.trim() + "\n";
    });
    return u + "\n";
  }
  if (name === "OL") {
    var o = "";
    var n = 1;
    node.querySelectorAll(":scope > li").forEach(function (li) {
      o += n + ". " + li.textContent.trim() + "\n";
      n++;
    });
    return o + "\n";
  }
  if (name === "HR") return "\n---\n\n";
  if (name === "IMG") {
    var alt = node.getAttribute("alt") || "image";
    var src = node.getAttribute("src") || "";
    return "![" + alt + "](" + src + ")\n\n";
  }
  if (name === "A") {
    var href = node.getAttribute("href") || "";
    return "[" + inner + "](" + href + ")";
  }
  return inner;
}

window.mduHtmlToMarkdown = function (rootEl) {
  if (!rootEl) return "";
  var md = "";
  for (var i = 0; i < rootEl.childNodes.length; i++) {
    md += mduWalkHtmlToMarkdown(rootEl.childNodes[i]);
  }
  return md.replace(/\n{3,}/g, "\n\n").trim() + "\n";
};

window.mduSyncPreviewEditorToSource = function () {
  var pe = document.getElementById("previewEditor");
  var sp = document.getElementById("previewPane");
  var root = null;
  if (pe && (document.activeElement === pe || pe.contains(document.activeElement))) root = pe;
  else if (sp && (document.activeElement === sp || sp.contains(document.activeElement))) root = sp;
  else root = pe || sp;
  var ta = mduGetSourceTextarea();
  if (!root || !ta) return null;
  var md = window.mduHtmlToMarkdown(root);
  ta.value = md;
  ta.dispatchEvent(new Event("input", { bubbles: true }));
  return md;
};

window.mduPreviewSetFromMarkdown = function (md, html) {
  var pe = document.getElementById("previewEditor");
  var ta = mduGetSourceTextarea();
  if (!pe || !ta) return;
  if (document.activeElement === pe || pe.contains(document.activeElement)) return;
  ta.value = md;
  if (pe.innerHTML !== html) {
    pe.innerHTML = html;
    if (window.mduRenderMath) window.mduRenderMath("#previewEditor");
  }
};

window.mduSplitPreviewSetFromMarkdown = function (md, html) {
  var pane = document.getElementById("previewPane");
  var ta = mduGetSourceTextarea();
  if (!pane || !ta) return;
  if (document.activeElement === pane || pane.contains(document.activeElement)) return;
  var ratio = 0;
  if (pane.scrollHeight > pane.clientHeight) {
    ratio = pane.scrollTop / Math.max(1, pane.scrollHeight - pane.clientHeight);
  }
  if (pane.innerHTML !== html) {
    pane.innerHTML = html;
    if (window.mduRenderMath) window.mduRenderMath("#previewPane");
    pane.scrollTop = ratio * Math.max(0, pane.scrollHeight - pane.clientHeight);
    if (window.mduSyncSplitScrollFromSource) window.mduSyncSplitScrollFromSource();
  }
};

window.mduSetSourceEditorValue = function (md) {
  var ta = mduGetSourceTextarea();
  if (!ta) return;
  if (document.activeElement === ta) return;
  ta.value = md || "";
};

function mduApplyPreviewCommand(before, after, placeholder) {
  var pe = mduActivePreviewEl();
  if (!pe) return null;
  pe.focus();
  var sel = window.getSelection();
  var selected = sel && sel.rangeCount ? sel.toString() : "";
  if (!selected) selected = placeholder || "";
  if (before === "**" && after === "**") document.execCommand("bold");
  else if (before === "*" && after === "*") document.execCommand("italic");
  else if (before === "***" && after === "***") {
    document.execCommand("bold");
    document.execCommand("italic");
  } else if (before === "~~" && after === "~~") document.execCommand("strikeThrough");
  else if (before === "`" && after === "`") document.execCommand("insertHTML", false, "<code>" + selected.replace(/</g, "&lt;") + "</code>");
  else if (before === "# ") document.execCommand("formatBlock", false, "h1");
  else if (before === "## ") document.execCommand("formatBlock", false, "h2");
  else if (before === "### ") document.execCommand("formatBlock", false, "h3");
  else if (before === "#### ") document.execCommand("formatBlock", false, "h4");
  else if (before === "##### ") document.execCommand("formatBlock", false, "h5");
  else if (before === "###### ") document.execCommand("formatBlock", false, "h6");
  else if (before === "> ") document.execCommand("formatBlock", false, "blockquote");
  else if (before === "- " || before === "1. " || before === "- [ ] ") document.execCommand("insertUnorderedList");
  else if (before === "![" && after.indexOf("](") !== -1) {
    var url = window.prompt("Image URL (https:// or leave blank to paste with Ctrl+V)", "https://");
    if (url) {
      document.execCommand(
        "insertHTML",
        false,
        '<img alt="' + selected.replace(/"/g, "") + '" src="' + url.replace(/"/g, "") + '" />'
      );
    }
  } else if (before === "[" && after.indexOf("](") !== -1) {
    var link = window.prompt("Link URL", "https://");
    if (link) {
      document.execCommand(
        "insertHTML",
        false,
        '<a href="' + link.replace(/"/g, "") + '">' + (selected || "link") + "</a>"
      );
    }
  } else document.execCommand("insertText", false, before + selected + after);
  return window.mduSyncPreviewEditorToSource();
}

window.mduApplyEditorCommand = function (editorId, before, after, placeholder) {
  if (mduPreviewEditorActive()) {
    var md = mduApplyPreviewCommand(before, after, placeholder);
    var ta = mduGetSourceTextarea();
    return ta ? ta.value : md;
  }
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
  if (el && el.tagName === "TEXTAREA") return el.value;
  return null;
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

  var textBefore = el.value.slice(0, o);
  var lineStart = el.value.lastIndexOf("\n", o - 1) + 1;
  var lineEnd = el.value.indexOf("\n", o);
  if (lineEnd === -1) lineEnd = el.value.length;
  var headingLine = el.value.slice(lineStart, lineEnd);
  var hm = headingLine.match(/^#{1,6}\s+(.*)/);

  var style = window.getComputedStyle(el);
  var lineHeight =
    parseFloat(style.lineHeight) ||
    parseFloat(style.fontSize) * 1.35 ||
    20;
  var lineIndex = textBefore.split("\n").length;
  var scrollTop = Math.max(0, (lineIndex - 2) * lineHeight);
  el.scrollTop = scrollTop;

  var previewSurface = document.querySelector(".editor-surface.preview-mode");
  if (previewSurface) {
    previewSurface.scrollTop = scrollTop;
  }

  var preview =
    document.getElementById("previewEditor") || document.getElementById("previewPane");
  if (preview && hm) {
    var title = hm[1].trim();
    var heads = preview.querySelectorAll("h1,h2,h3,h4,h5,h6");
    for (var i = 0; i < heads.length; i++) {
      if (heads[i].textContent.trim() === title) {
        heads[i].scrollIntoView({ block: "start", behavior: "smooth" });
        if (previewSurface) {
          var rect = heads[i].getBoundingClientRect();
          var srect = previewSurface.getBoundingClientRect();
          previewSurface.scrollTop += rect.top - srect.top - 12;
        } else {
          var splitPreview = preview.closest(".preview");
          if (splitPreview) {
            var r2 = heads[i].getBoundingClientRect();
            var s2 = splitPreview.getBoundingClientRect();
            splitPreview.scrollTop += r2.top - s2.top - 12;
          }
        }
        break;
      }
    }
  }
};

window.mduSyncPreviewLayout = function () {
  var surface = document.querySelector(".editor-surface.preview-mode");
  var stack = document.getElementById("previewModeStack");
  var preview = document.getElementById("previewPane");
  var ta = document.getElementById("mduSourceEditor");
  if (!surface || !stack || !preview || !ta) return;
  var h = Math.max(preview.scrollHeight, surface.clientHeight, 320);
  stack.style.minHeight = h + "px";
  ta.style.height = h + "px";
  if (!ta._mduPreviewScrollBound) {
    ta._mduPreviewScrollBound = true;
    var syncing = false;
    ta.addEventListener("scroll", function () {
      if (syncing) return;
      syncing = true;
      surface.scrollTop = ta.scrollTop;
      syncing = false;
    });
    surface.addEventListener("scroll", function () {
      if (syncing) return;
      syncing = true;
      ta.scrollTop = surface.scrollTop;
      syncing = false;
    });
  }
};

window.mduInitOutlineResize = function (handleId, panelId, storageKey) {
  var handle = document.getElementById(handleId);
  var panel = document.getElementById(panelId);
  if (!handle || !panel) return;
  if (handle._mduOutlineBound) return;
  handle._mduOutlineBound = true;
  var dragging = false;
  var startY = 0;
  var startH = 0;
  function onDown(ev) {
    dragging = true;
    startY = ev.clientY != null ? ev.clientY : ev.touches && ev.touches[0] ? ev.touches[0].clientY : 0;
    startH = panel.getBoundingClientRect().height;
    ev.preventDefault();
    document.body.classList.add("mdu-resizing-outline");
  }
  function onMove(ev) {
    if (!dragging) return;
    var y = ev.clientY != null ? ev.clientY : ev.touches && ev.touches[0] ? ev.touches[0].clientY : startY;
    var delta = startY - y;
    var next = Math.max(72, Math.min(window.innerHeight * 0.55, startH + delta));
    panel.style.height = next + "px";
  }
  function onUp() {
    if (!dragging) return;
    dragging = false;
    document.body.classList.remove("mdu-resizing-outline");
    var h = panel.getBoundingClientRect().height;
    try {
      localStorage.setItem(storageKey, String(Math.round(h)));
    } catch (e) {}
    window.dispatchEvent(
      new CustomEvent("mdu-outline-height", { detail: Math.round(h) })
    );
  }
  handle.addEventListener("mousedown", onDown);
  handle.addEventListener("touchstart", onDown, { passive: false });
  window.addEventListener("mousemove", onMove);
  window.addEventListener("touchmove", onMove, { passive: false });
  window.addEventListener("mouseup", onUp);
  window.addEventListener("touchend", onUp);
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
  if (action === "paste-image") {
    var el = document.getElementById(editorId);
    if (!el || el.tagName !== "TEXTAREA") return null;
    mduPasteImageFromClipboard(el);
    return el.value;
  }
  if (action === "insert-image-url") {
    var el2 = document.getElementById(editorId);
    if (!el2 || el2.tagName !== "TEXTAREA") return null;
    var url = window.prompt("Image URL (https://)", "https://");
    if (!url) return el2.value;
    var alt = window.prompt("Short description (optional)", "image") || "image";
    return mduInsertAtTextarea(el2, "![" + alt + "](" + url + ")\n\n");
  }
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

function mduInsertImageMarkdown(targetEl, src, alt) {
  alt = alt || "image";
  var md = "![" + alt + "](" + src + ")\n\n";
  if (mduPreviewEditorActive()) {
    document.execCommand(
      "insertHTML",
      false,
      '<img alt="' + alt.replace(/"/g, "") + '" src="' + src.replace(/"/g, "") + '" />'
    );
    return window.mduSyncPreviewEditorToSource();
  }
  if (targetEl && targetEl.tagName === "TEXTAREA") {
    mduInsertAtTextarea(targetEl, md);
    return targetEl.value;
  }
  return null;
}

function mduPasteImageFromClipboard(targetEl) {
  if (!navigator.clipboard || !navigator.clipboard.read) {
    window.alert("Copy an image, click in the document, then press Ctrl+V.");
    return;
  }
  navigator.clipboard
    .read()
    .then(function (items) {
      for (var i = 0; i < items.length; i++) {
        var item = items[i];
        for (var t = 0; t < item.types.length; t++) {
          if (item.types[t].indexOf("image") === -1) continue;
          item.getType(item.types[t]).then(function (blob) {
            var reader = new FileReader();
            reader.onload = function () {
              mduInsertImageMarkdown(targetEl, reader.result, "pasted image");
            };
            reader.readAsDataURL(blob);
          });
          return;
        }
      }
      window.alert("No image found on the clipboard. Copy a picture first, then try again.");
    })
    .catch(function () {
      window.alert("Could not read the clipboard. Try Ctrl+V in the editor instead.");
    });
}

function mduBindImagePaste(el) {
  if (!el || el._mduImagePasteBound) return;
  el._mduImagePasteBound = true;
  el.addEventListener("paste", function (ev) {
    var cd = ev.clipboardData;
    if (!cd || !cd.items) return;
    for (var i = 0; i < cd.items.length; i++) {
      var it = cd.items[i];
      if (it.type.indexOf("image") === -1) continue;
      ev.preventDefault();
      var blob = it.getAsFile();
      if (!blob) continue;
      var reader = new FileReader();
      reader.onload = function () {
        if (el.id === "previewEditor" || mduPreviewEditorActive()) {
          mduInsertImageMarkdown(el, reader.result, "pasted image");
        } else {
          mduInsertImageMarkdown(mduGetSourceTextarea(), reader.result, "pasted image");
        }
      };
      reader.readAsDataURL(blob);
      return;
    }
  });
}

window.mduInitImagePaste = function () {
  if (window._mduImagePasteInit) return;
  window._mduImagePasteInit = true;
  var bindAll = function () {
    var ta = mduGetSourceTextarea();
    if (ta) mduBindImagePaste(ta);
    var pe = document.getElementById("previewEditor");
    if (pe) mduBindImagePaste(pe);
  };
  bindAll();
  window.addEventListener("mdu-view-changed", bindAll);
  setTimeout(bindAll, 500);
};

window.mduInitPreviewEditor = function () {
  if (window._mduPreviewEditorInit) return;
  window._mduPreviewEditorInit = true;
  var bind = function () {
    var pe = document.getElementById("previewEditor");
    if (!pe || pe._mduPreviewBound) return;
    pe._mduPreviewBound = true;
    mduBindImagePaste(pe);
    var syncTimer = null;
    var scheduleSync = function () {
      if (syncTimer) clearTimeout(syncTimer);
      syncTimer = setTimeout(function () {
        window.mduSyncPreviewEditorToSource();
        if (window.mduRenderMath) window.mduRenderMath("#previewEditor");
      }, 120);
    };
    pe.addEventListener("input", scheduleSync);
    pe.addEventListener("blur", function () {
      window.mduSyncPreviewEditorToSource();
    });
  };
  bind();
  window.addEventListener("mdu-view-changed", bind);
  setTimeout(bind, 400);
};

window.mduInitUiZoom = function () {
  var shell = document.getElementById("appShell");
  if (!shell || shell._mduZoomBound) return;
  shell._mduZoomBound = true;
  var key = "mdu_ui_zoom";
  var z = 1;
  try {
    var saved = parseFloat(localStorage.getItem(key) || "1");
    if (!isNaN(saved) && saved >= 0.5 && saved <= 2.5) z = saved;
  } catch (e) {}
  shell.style.zoom = "";
  shell.style.setProperty("--mdu-ui-zoom", String(z));
  window.dispatchEvent(new CustomEvent("mdu-ui-zoom", { detail: z }));
  document.addEventListener(
    "wheel",
    function (ev) {
      if (!ev.ctrlKey) return;
      if (!shell.contains(ev.target)) return;
      ev.preventDefault();
      z = Math.max(0.5, Math.min(2.5, z + (ev.deltaY < 0 ? 0.06 : -0.06)));
      z = Math.round(z * 100) / 100;
      shell.style.setProperty("--mdu-ui-zoom", String(z));
      try {
        localStorage.setItem(key, String(z));
      } catch (e2) {}
      window.dispatchEvent(new CustomEvent("mdu-ui-zoom", { detail: z }));
    },
    { passive: false }
  );
};

window.mduResetUiZoom = function () {
  var shell = document.getElementById("appShell");
  if (!shell) return;
  shell.style.zoom = "";
  shell.style.setProperty("--mdu-ui-zoom", "1");
  try {
    localStorage.setItem("mdu_ui_zoom", "1");
  } catch (e) {}
  window.dispatchEvent(new CustomEvent("mdu-ui-zoom", { detail: 1 }));
};

window.mduApplyTheme = function (theme) {
  var t = theme || "light";
  document.documentElement.setAttribute("data-mdu-theme", t);
  try {
    localStorage.setItem("mdu_theme_v1", t);
  } catch (e) {}
};

window.mduSyncSplitScrollFromSource = function () {
  var ta = document.getElementById("mduSourceEditor");
  var prev = document.getElementById("previewPane");
  if (!ta || !prev) return;
  var maxFrom = Math.max(1, ta.scrollHeight - ta.clientHeight);
  var ratio = ta.scrollTop / maxFrom;
  var maxTo = Math.max(0, prev.scrollHeight - prev.clientHeight);
  prev.scrollTop = ratio * maxTo;
};

window.mduInitSplitScrollSync = function () {
  var bind = function () {
    var surface = document.getElementById("splitEditorSurface");
    var ta = document.getElementById("mduSourceEditor");
    var prev = document.getElementById("previewPane");
    if (!surface || !ta || !prev) return;
    if (ta._mduSplitSyncBound) return;
    if (!surface.classList.contains("split")) return;
    ta._mduSplitSyncBound = true;
    var syncing = false;
    function sync(from, to) {
      if (syncing) return;
      syncing = true;
      var maxFrom = Math.max(1, from.scrollHeight - from.clientHeight);
      var ratio = from.scrollTop / maxFrom;
      var maxTo = Math.max(0, to.scrollHeight - to.clientHeight);
      to.scrollTop = ratio * maxTo;
      syncing = false;
    }
    ta.addEventListener("scroll", function () {
      sync(ta, prev);
    });
    prev.addEventListener("scroll", function () {
      sync(prev, ta);
    });
    ta.addEventListener("input", function () {
      setTimeout(function () {
        window.mduSyncSplitScrollFromSource();
      }, 30);
    });
  };
  bind();
  window.addEventListener("mdu-view-changed", function () {
    var ta = document.getElementById("mduSourceEditor");
    if (ta) ta._mduSplitSyncBound = false;
    setTimeout(bind, 120);
  });
  setTimeout(bind, 200);
};

window.mduInitSplitPreviewEditor = function () {
  var bind = function () {
    var pane = document.getElementById("previewPane");
    if (!pane || pane.getAttribute("contenteditable") !== "true" || pane._mduSplitPreviewBound) return;
    pane._mduSplitPreviewBound = true;
    mduBindImagePaste(pane);
    var syncTimer = null;
    var scheduleSync = function () {
      if (syncTimer) clearTimeout(syncTimer);
      syncTimer = setTimeout(function () {
        window.mduSyncPreviewEditorToSource();
        if (window.mduRenderMath) window.mduRenderMath("#previewPane");
      }, 100);
    };
    pane.addEventListener("input", scheduleSync);
    pane.addEventListener("blur", function () {
      window.mduSyncPreviewEditorToSource();
    });
  };
  bind();
  window.addEventListener("mdu-view-changed", function () {
    var pane = document.getElementById("previewPane");
    if (pane) pane._mduSplitPreviewBound = false;
    setTimeout(bind, 150);
  });
  setTimeout(bind, 250);
};

window.mduInitMobileLayout = function () {
  if (window._mduMobileInit) return;
  window._mduMobileInit = true;
  function applyMobile() {
    var mobile =
      window.matchMedia("(max-width: 768px)").matches ||
      /Android|iPhone|iPad|Mobile/i.test(navigator.userAgent || "") ||
      document.documentElement.classList.contains("mdu-mobile-ready");
    document.body.classList.toggle("mdu-mobile", mobile);
    if (mobile) {
      var split = document.getElementById("splitEditorSurface");
      if (split && !split.classList.contains("show-source") && !split.classList.contains("show-preview")) {
        split.classList.add("show-preview");
      }
    }
  }
  applyMobile();
  window.addEventListener("resize", applyMobile);
  window.addEventListener("mdu-view-changed", applyMobile);
  document.addEventListener("click", function (ev) {
    var btn = ev.target.closest(".mobile-split-btn");
    if (!btn) return;
    var surface = document.getElementById("splitEditorSurface");
    if (!surface) return;
    var mode = btn.getAttribute("data-mobile-split");
    surface.querySelectorAll(".mobile-split-btn").forEach(function (b) {
      b.classList.toggle("active", b === btn);
    });
    surface.classList.remove("show-source", "show-preview");
    if (mode === "source") surface.classList.add("show-source");
    else surface.classList.add("show-preview");
  });
  try {
    var theme = localStorage.getItem("mdu_theme_v1") || "light";
    window.mduApplyTheme(theme);
  } catch (e) {}
};
