# Korlang Web Infrastructure & Installer Roadmap

This document tracks the development of the **Korlang Website** and **Global Installer**. Since we are using a serverless approach, this project will be hosted on **GitHub Pages**, and binaries will be served via **GitHub Releases**.

**Target Folder:** `../Korlang-Site` (Sibling to `Korlang-Compiler`)

---

## 🌐 Phase S1: Site Structure & Landing Page
**Goal:** Create a modern, static landing page to market the language and provide documentation.

- [ ] **S1.1 Initialize Project:**
    - Create a new folder `Korlang-Site` and initialize a git repo.
    - Create `index.html` (Landing Page) and `css/style.css`.
- [ ] **S1.2 Documentation Section:**
    - Create `docs/` folder structure.
    - Implement a simple Markdown-to-HTML renderer (or use Jekyll/Hugo) for the "Korlang Book".
- [ ] **S1.3 "Get Started" Page:**
    - Display the one-line install command clearly.
    - Provide manual download links for Linux, macOS, and Windows.

---

## 📦 Phase S2: The Universal Installer (`korup`)
**Goal:** Create the scripts that `curl | sh` will execute to install the CLI.

- [ ] **S2.1 Linux/macOS Script (`install.sh`):**
    - [ ] Detect OS (`uname -s`) and Architecture (`uname -m`).
    - [ ] Determine the latest version from GitHub API (`https://api.github.com/repos/project-korlang/korlang/releases/latest`).
    - [ ] Download the correct `.tar.gz` from GitHub Releases.
    - [ ] Extract to `~/.korlang/bin`.
    - [ ] Update `~/.bashrc` or `~/.zshrc` to add `~/.korlang/bin` to PATH.
- [ ] **S2.2 Windows Script (`install.ps1`):**
    - [ ] Perform similar logic using PowerShell for Windows users.
- [ ] **S2.3 Version Management:**
    - [ ] Ensure the script can handle `korlang update` by re-running the fetch logic.

---

## 🚀 Phase S3: Deployment Pipeline
**Goal:** Automate the hosting of the site and the distribution of the scripts.

- [ ] **S3.1 GitHub Pages Configuration:**
    - [ ] Configure the repo to serve from the `/docs` folder or root.
    - [ ] (Optional) Configure Custom Domain (CNAME) if `korlang.org` is purchased later.
- [ ] **S3.2 Binary Release Workflow (CI/CD):**
    - [ ] Update `Korlang-Compiler` CI to upload compiled binaries to GitHub Releases on tag push.
    - [ ] Naming convention: `korlang-<version>-<os>-<arch>.tar.gz`.

---

## 🔗 Architecture Overview

1. **User runs:** `curl -fsSL https:/project-korlang.github.io/korlang/install.sh | sh`
2. **Script runs:**
   - Checks system info.
   - Fetches binary from `github.com/project-korlang/korlang/releases/...`
   - Installs to `~/.korlang/`.
3. **Result:** User types `korlang` and it works.
