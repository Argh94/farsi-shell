use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ar_reshaper::ArabicReshaper;

// بررسی اینکه آیا کاراکتر متعلق به بلوک‌های عربی/فارسی است یا خیر
fn is_arabic_char(c: char) -> bool {
    let cp = c as u32;
    (0x0600..=0x06FF).contains(&cp) || // استاندارد عربی/فارسی
    (0x0750..=0x077F).contains(&cp) || // مکمل عربی
    (0x08A0..=0x08FF).contains(&cp) || // عربی Extended-A
    (0xFB50..=0xFDFF).contains(&cp) || // اشکال متصل A
    (0xFE70..=0xFEFF).contains(&cp)    // اشکال متصل B
}

// تشخیص اعداد انگلیسی، فارسی و عربی
fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || ('\u{0660}'..='\u{066F}').contains(&c) || ('\u{06F0}'..='\u{06F9}').contains(&c)
}

// معکوس کردن متن به شکل بصری با حفظ ترتیب صحیح اعداد از چپ به راست
fn visual_reverse(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut rev_chars = chars;
    rev_chars.reverse();
    
    let mut i = 0;
    while i < rev_chars.len() {
        if is_digit(rev_chars[i]) {
            let start = i;
            while i < rev_chars.len() && (is_digit(rev_chars[i]) || rev_chars[i] == '.' || rev_chars[i] == ',' || rev_chars[i] == ':') {
                i += 1;
            }
            // اعداد پیدا شده را مجدداً معکوس می‌کنیم تا ترتیب چپ‌به‌راست آن‌ها حفظ شود
            rev_chars[start..i].reverse();
        } else {
            i += 1;
        }
    }
    rev_chars.into_iter().collect()
}

// پردازش خطوط متنی و استخراج بلوک‌های متنی فارسی برای اصلاح
fn process_line(line: &str, reshaper: &ArabicReshaper) -> String {
    let mut result = String::new();
    let mut current_block = String::new();
    let mut in_arabic_block = false;

    for c in line.chars() {
        let is_arab = is_arabic_char(c);
        let is_neutral = c == ' ' || c.is_ascii_punctuation();

        if is_arab {
            if !in_arabic_block && !current_block.is_empty() {
                result.push_str(&current_block);
                current_block.clear();
            }
            in_arabic_block = true;
            current_block.push(c);
        } else if is_neutral && in_arabic_block {
            current_block.push(c);
        } else {
            if in_arabic_block {
                result.push_str(&process_arabic_block(&current_block, reshaper));
                current_block.clear();
                in_arabic_block = false;
            }
            current_block.push(c);
        }
    }

    if !current_block.is_empty() {
        if in_arabic_block {
            result.push_str(&process_arabic_block(&current_block, reshaper));
        } else {
            result.push_str(&current_block);
        }
    }

    result
}

fn process_arabic_block(block: &str, reshaper: &ArabicReshaper) -> String {
    let reshaped = reshaper.reshape(block);
    visual_reverse(&reshaped)
}

// ماشین وضعیت برای نادیده گرفتن کدهای کنترلی ترمینال (ANSI/CSI escape codes)
enum ParserState {
    Normal,
    Escape,
    Csi,
}

struct TermParser {
    state: ParserState,
    text_buffer: Vec<u8>,
    reshaper: ArabicReshaper,
}

impl TermParser {
    fn new() -> Self {
        Self {
            state: ParserState::Normal,
            text_buffer: Vec::new(),
            reshaper: ArabicReshaper::default(),
        }
    }

    fn handle_bytes(&mut self, bytes: &[u8], stdout: &mut impl Write) -> io::Result<()> {
        for &b in bytes {
            match self.state {
                ParserState::Normal => {
                    if b == 0x1B { // کلید اسکپ (شروع کدهای رنگ/حرکت مکان‌نما)
                        self.flush_complete_utf8(stdout)?;
                        self.state = ParserState::Escape;
                        stdout.write_all(&[b])?;
                    } else {
                        self.text_buffer.push(b);
                    }
                }
                ParserState::Escape => {
                    stdout.write_all(&[b])?;
                    if b == b'[' {
                        self.state = ParserState::Csi;
                    } else {
                        self.state = ParserState::Normal;
                    }
                }
                ParserState::Csi => {
                    stdout.write_all(&[b])?;
                    if (0x40..=0x7E).contains(&b) { // انتهای توالی متغیرهای CSI
                        self.state = ParserState::Normal;
                    }
                }
            }
        }
        self.flush_complete_utf8(stdout)?;
        Ok(())
    }

    fn flush_complete_utf8(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        if self.text_buffer.is_empty() {
            return Ok(());
        }
        let mut len = self.text_buffer.len();
        while len > 0 {
            match std::str::from_utf8(&self.text_buffer[..len]) {
                Ok(text) => {
                    let processed = process_line(text, &self.reshaper);
                    stdout.write_all(processed.as_bytes())?;
                    self.text_buffer.drain(..len);
                    break;
                }
                Err(e) => {
                    if e.valid_up_to() > 0 {
                        let text = std::str::from_utf8(&self.text_buffer[..e.valid_up_to()]).unwrap();
                        let processed = process_line(text, &self.reshaper);
                        stdout.write_all(processed.as_bytes())?;
                        self.text_buffer.drain(..e.valid_up_to());
                        break;
                    } else if self.text_buffer.len() > 4 {
                        stdout.write_all(&[self.text_buffer[0]])?;
                        self.text_buffer.remove(0);
                        len = self.text_buffer.len();
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // پیدا کردن مسیر شل سیستم
    let shell = std::env::var("SHELL").unwrap_or_else(|_| {
        if std::path::Path::new("/data/data/com.termux/files/usr/bin/bash").exists() {
            "/data/data/com.termux/files/usr/bin/bash".to_string()
        } else {
            "/system/bin/sh".to_string()
        }
    });

    let pty_system = NativePtySystem::default();
    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    
    let pair = pty_system.openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new(&shell);
    for (key, val) in std::env::vars() {
        cmd.env(key, val);
    }
    
    // یک متغیر محیطی برای شناسایی ساب‌شل می‌سازیم تا از ایجاد حلقه تکرار بی‌نهایت جلوگیری شود
    cmd.env("FARSI_SHELL_ACTIVE", "1");

    let mut child = pair.slave.spawn_command(cmd)?;
    drop(pair.slave);

    // بردن ترمینال به حالت خام (Raw) تا کلیدها مستقیماً به شل فرستاده شوند
    enable_raw_mode()?;
    
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;
    let (tx, rx) = mpsc::channel();

    // ترد ورودی: خواندن از کیبورد و فرستادن به شل داخلی
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0u8; 1024];
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if writer.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    let _ = writer.flush();
                }
                Err(_) => break,
            }
        }
        let _ = tx_clone.send(());
    });

    // ترد خروجی: خواندن از شل، پارس کدهای ANSI و ویرایش بلادرنگ فارسی
    thread::spawn(move || {
        let mut stdout = io::stdout();
        let mut buffer = [0u8; 4096];
        let mut parser = TermParser::new();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if parser.handle_bytes(&buffer[..n], &mut stdout).is_err() {
                        break;
                    }
                    let _ = stdout.flush();
                }
                Err(_) => break,
            }
        }
        let _ = tx.send(());
    });

    // منتظر ماندن برای پایان کار شل یا خروج کاربر
    loop {
        if rx.try_recv().is_ok() {
            break;
        }
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
        thread::sleep(std::time::Duration::from_millis(50));
    }

    let _ = disable_raw_mode();
    Ok(())
}
