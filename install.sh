#!/usr/bin/env bash

set -e

echo "⏳ در حال کامپایل برنامه فارسی‌ساز راست..."
cargo build --release

# انتقال فایل باینری خروجی به مسیر باینری‌های ترموکس
DEST_DIR="$PREFIX/bin"
mkdir -p "$DEST_DIR"
cp target/release/farsi-shell "$DEST_DIR/farsi-shell"
chmod +x "$DEST_DIR/farsi-shell"

echo "✅ فایل باینری با موفقیت در مسیر $DEST_DIR/farsi-shell کپی شد."

# تشخیص نوع شل پیش‌فرض و اضافه کردن گارد جلوگیری از لوپ
SHELL_CONFIG=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
else
    # اگر فایلی وجود نداشت، یک فایل .bashrc می‌سازیم
    SHELL_CONFIG="$HOME/.bashrc"
    touch "$SHELL_CONFIG"
fi

# بررسی اینکه آیا قبلاً پیکربندی در فایل شل اعمال شده است یا خیر
if ! grep -q "FARSI_SHELL_ACTIVE" "$SHELL_CONFIG"; then
    echo "⚙️ در حال اضافه کردن اسکریپت راه‌انداز به $SHELL_CONFIG ..."
    cat << 'EOF' >> "$SHELL_CONFIG"

# Farsi-Shell Auto Launcher Guard
if [ -z "$FARSI_SHELL_ACTIVE" ] && [ -x "$(command -v farsi-shell)" ]; then
    exec farsi-shell
fi
EOF
    echo "🎉 تنظیمات شل به‌روزرسانی شد."
else
    echo "ℹ️ تنظیمات راه‌اندازی از قبل در $SHELL_CONFIG وجود دارد."
fi

echo "✨ نصب با موفقیت انجام شد! لطفاً ترموکس خود را ببندید و دوباره باز کنید."
