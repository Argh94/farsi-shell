a/farsi-shell\install.sh → b/farsi-shell\install.sh
@@ -0,0 +1,290 @@
+#!/data/data/com.termux/files/usr/bin/bash
+#
+# farsi-shell installer for Termux
+#
+# Usage:
+#   curl -sSL https://raw.githubusercontent.com/Argh94/farsi-shell/main/install.sh | bash
+#
+# Or download and run manually:
+#   chmod +x install.sh
+#   ./install.sh
+#
+
+set -e
+
+# Colors for output
+RED='\033[0;31m'
+GREEN='\033[0;32m'
+YELLOW='\033[1;33m'
+BLUE='\033[0;34m'
+NC='\033[0m' # No Color
+
+# GitHub repository
+REPO_URL="https://github.com/Argh94/farsi-shell"
+RELEASE_URL="${REPO_URL}/releases/latest/download"
+
+# Installation directory (Termux bin directory)
+INSTALL_DIR="${PREFIX}/bin"
+if [ -z "$PREFIX" ]; then
+    INSTALL_DIR="/data/data/com.termux/files/usr/bin"
+fi
+
+# Detect architecture
+detect_arch() {
+    local arch=$(uname -m)
+    case "$arch" in
+        aarch64|arm64)
+            echo "aarch64"
+            ;;
+        armv7*|armv8*|arm)
+            echo "arm"
+            ;;
+        x86_64)
+            echo "x86_64"
+            ;;
+        i*86)
+            echo "i686"
+            ;;
+        *)
+            echo "unknown"
+            ;;
+    esac
+}
+
+# Print banner
+print_banner() {
+    echo -e "${BLUE}"
+    echo "╔══════════════════════════════════════════════════════════╗"
+    echo "║           farsi-shell Installer for Termux              ║"
+    echo "╠══════════════════════════════════════════════════════════╣"
+    echo "║  Persian/Arabic text display with correct reshaping     ║"
+    echo "║  and BiDi support for Termux terminal                   ║"
+    echo "╚══════════════════════════════════════════════════════════╝"
+    echo -e "${NC}"
+}
+
+# Check if running in Termux
+check_termux() {
+    if [ ! -d "/data/data/com.termux" ]; then
+        echo -e "${YELLOW}Warning: This installer is designed for Termux.${NC}"
+        echo "You can still install manually by building from source."
+        read -p "Continue anyway? (y/N) " -n 1 -r
+        echo
+        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
+            exit 1
+        fi
+    fi
+}
+
… omitted 212 diff line(s) across 1 additional file(s)/section(s)
