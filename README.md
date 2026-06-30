# farsi-shell
a/farsi-shell\README.md → b/farsi-shell\README.md
@@ -0,0 +1,280 @@
+# farsi-shell 🇮🇷
+
+**Persian/Arabic text display for Termux with character reshaping and BiDi support**
+
+[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
+
+## Problem
+
+In Termux (Android terminal emulator), Persian and Arabic text is displayed incorrectly:
+- Letters appear in their **isolated forms** (not connected)
+- Text direction is **left-to-right** (LTR) instead of **right-to-left** (RTL)
+- Mixed Persian/English text is jumbled and unreadable
+
+## Solution
+
+`farsi-shell` is a PTY (pseudo-terminal) wrapper that sits between Termux and your shell, intercepting output to:
+
+1. **Reshape** Arabic/Persian characters (connect letter forms correctly)
+2. **Reorder** text using the BiDi algorithm (display RTL text correctly)
+3. **Preserve** ANSI escape sequences (colors, cursor movement, etc.)
+
+## Features
+
+- ✅ **Character Reshaping** - Connects Arabic/Persian letters in their correct contextual forms (isolated, initial, medial, final)
+- ✅ **BiDi Text Reordering** - Handles mixed LTR/RTL text correctly
+- ✅ **ANSI Support** - Preserves terminal colors, styles, and cursor movement
+- ✅ **UTF-8 Handling** - Correctly processes multi-byte characters
+- ✅ **Streaming Processing** - Real-time text processing with minimal latency
+- ✅ **Lightweight** - Single binary, minimal memory footprint
+- ✅ **Easy Install** - One-command installation script
+
+## Installation
+
+### Quick Install (Recommended)
+
+```bash
+curl -sSL https://raw.githubusercontent.com/youruser/farsi-shell/main/install.sh | bash
+```
+
+### Manual Install
+
+1. Download the binary for your architecture:
+   ```bash
+   # For most Android devices (aarch64/arm64)
+   curl -LO https://github.com/youruser/farsi-shell/releases/latest/download/farsi-shell-aarch64-linux-android
+
+   # For older devices (arm)
+   curl -LO https://github.com/youruser/farsi-shell/releases/latest/download/farsi-shell-arm-linux-androideabi
+
+   # For emulators (x86_64)
+   curl -LO https://github.com/youruser/farsi-shell/releases/latest/download/farsi-shell-x86_64-linux-android
+   ```
+
+2. Make executable and install:
+   ```bash
+   chmod +x farsi-shell-*
+   mv farsi-shell-* $PREFIX/bin/farsi-shell
+   ```
+
+3. (Optional) Add to shell startup:
+   ```bash
+   echo 'if [ -z "$FARSI_SHELL" ] && [ -t 0 ]; then exec farsi-shell; fi' >> ~/.bashrc
+   ```
+
+### Build from Source
+
+1. Install Rust:
+   ```bash
+   pkg install rust
+   ```
+
+2. Clone and build:
+   ```bash
+   git clone https://github.com/youruser/farsi-shell.git
+   cd farsi-shell
+   cargo build --release
+   ```
+
… omitted 202 diff line(s) across 1 additional file(s)/section(s)
