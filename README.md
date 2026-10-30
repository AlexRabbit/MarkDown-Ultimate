<p align="center">
  <strong style="font-size: 1.5em;">MarkDown Ultimate</strong><br/>
  <em>Your notes, folders, and preview — in one calm place.</em>
</p>

<p align="center">
  <a href="https://alexrabbit.github.io/MarkDown-Ultimate/"><strong>Open the app online</strong></a>
  &nbsp;·&nbsp;
  <a href="https://github.com/AlexRabbit/MarkDown-Ultimate/releases">Download for Windows & Android</a>
</p>

---
![Preview](https://raw.githubusercontent.com/AlexRabbit/MarkDown-Ultimate/refs/heads/main/PREVIEW.png)
---
## What it is

**MarkDown Ultimate** is a writing app for Markdown: the plain, readable format used for notes, docs, and README files. You work in your browser or in a small desktop window — no account required for the online version.

---

## What it does

You keep **many notes at once**, organized in **folders**, with a **live preview** of how they will look when shared or printed. You can export a single note, save your whole workspace as a backup, or print to PDF from the preview.

Everything feels like one desk: files on the left, writing in the middle, optional tools on the right.

---

## Features and layout

| Area | What you get |
| --- | --- |
| **Left — Files** | Folder tree, scroll when you have lots of notes. **📄** creates a new note; **+** creates a new folder. Use **⋮** on a file or folder to rename or remove. Drag items to reorder or nest. |
| **Center — Editor** | Write Markdown with a **Markdown / Both / Preview** switch at the top. Preview shows headings, lists, tables, math, and more. |
| **Right — Tools** (optional) | Open **⟨ Tools** for formatting shortcuts, find/replace, and a shortcut cheat sheet. |
| **Below the tree — Outline** | Click a heading to jump in the long notes. |
| **Top bar** | **Open** and **Save** for files and full-workspace backups; **How to Use** starts a guided tour when *you* choose. |
| **Bottom** | Word and character counts. |

**Helpful extras:** PDF via your browser’s print dialog, diagnostics copy for support, and a guided tour that never pops up on its own.

---

## Use it online (nothing to install)

Open this link in Chrome, Edge, or Firefox:

### [https://alexrabbit.github.io/MarkDown-Ultimate/](https://alexrabbit.github.io/MarkDown-Ultimate/)

Your notes stay in **this browser** unless you export them. Bookmark the link for quick access.

---

## Download — Windows app, Android app, and zip

Official builds are published on **GitHub Releases** (free downloads):

| What | Where to get it |
| --- | --- |
| **Windows app** (`MarkDownUltimate.exe`) | [Latest release → MarkDownUltimate.exe](https://github.com/AlexRabbit/MarkDown-Ultimate/releases/latest/download/MarkDownUltimate.exe) |
| **Android app** (`MarkDown-Ultimate.apk`) | [Latest release → MarkDown-Ultimate.apk](https://github.com/AlexRabbit/MarkDown-Ultimate/releases/latest/download/MarkDown-Ultimate.apk) |
| **Portable zip** (folder you can unzip anywhere) | [Latest release → MarkDownUltimate-Portable.zip](https://github.com/AlexRabbit/MarkDown-Ultimate/releases/latest/download/MarkDownUltimate-Portable.zip) |

If a link says “Not Found,” open the [Releases page](https://github.com/AlexRabbit/MarkDown-Ultimate/releases) and pick the newest **v** tag — assets are attached there.

### After you clone or download this project

Everything is meant to live in **one project folder** (this repository). When you build locally:

| File | Location in your project |
| --- | --- |
| **Windows `.exe`** | `MarkDownUltimate.exe` in the **root** of this folder (created by `tools\portable-bundle.ps1`) |
| **Android `.apk`** (build output) | `release\artifacts\MarkDown-Ultimate.apk` |
| **Copies ready for GitHub upload** | `release\artifacts\` (exe, zip, apk) |

---

## What the `.bat` files do (Windows)

| File | What happens when you double‑click |
| --- | --- |
| **`Launch-MarkDownUltimate.bat`** | Starts a tiny local server and opens the app in your **default browser** at `http://127.0.0.1:8765/MarkDown-Ultimate/`. Leave the window open while you write. Best for development or if you prefer a browser tab. |
| **`Run-Portable-MarkDownUltimate.bat`** | Starts **`MarkDownUltimate.exe`** in the same folder — a **desktop window** with the app inside (no separate browser tab). Run `tools\portable-bundle.ps1` first if the exe is missing. |

---

## One folder — not two

You should only need **this repository folder** (for example `MarkDown Ultimate` on your PC).

An older setup copied a second folder with a similar name (`MarkDown-Ultimate` next to this one). That duplicate is **removed** when you run `tools\portable-bundle.ps1` or `tools\cleanup-release-duplicates.ps1`. The app, `dist`, optional embedded Python, and `MarkDownUltimate.exe` all update **here**.

---

## Publish your own copy online (light overview)

If you fork this project on GitHub:

1. Push your changes to the **`main`** branch.
2. Turn on **GitHub Pages** for the repo (Settings → Pages → deploy from **GitHub Actions**).
3. The workflow **Deploy GitHub Pages** builds the site; after a few minutes your fork gets a URL like `https://<your-user>.github.io/MarkDown-Ultimate/`.

To share **exe / apk** files, attach them to a **Release** on GitHub (Releases → Draft a new release → upload the files from `release\artifacts\`). You do not commit those large binaries into git.

---

## License

MIT — [AlexRabbit](https://github.com/AlexRabbit)

<p align="center"><sub>Made for people who like clear notes and simple tools.</sub></p>
