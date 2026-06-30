use crate::reshaper::reshape_line;
use crate::bidi::{has_rtl_chars, process_line};

enum TextChunk { AnsiSequence(String), Text(String) }

fn split_ansi_sequences(text: &str) -> Vec<TextChunk> {
    let mut chunks = Vec::new();
    let mut current_text = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if !current_text.is_empty() {
                chunks.push(TextChunk::Text(current_text.clone()));
                current_text.clear();
            }
            let mut sequence = String::from(ch);
            if let Some(&next) = chars.peek() {
                if next == '[' {
                    sequence.push(chars.next().unwrap());
                    while let Some(&next) = chars.peek() {
                        sequence.push(chars.next().unwrap());
                        if next.is_ascii_alphabetic() { break; }
                    }
                } else if next == ']' {
                    sequence.push(chars.next().unwrap());
                    while let Some(&next) = chars.peek() {
                        sequence.push(chars.next().unwrap());
                        if next == '\x07' || next == '\x1b' { break; }
                    }
                    if chars.peek() == Some(&'\\') { sequence.push(chars.next().unwrap()); }
                } else {
                    sequence.push(chars.next().unwrap_or('\0'));
                }
            }
            chunks.push(TextChunk::AnsiSequence(sequence));
        } else {
            current_text.push(ch);
        }
    }
    if !current_text.is_empty() { chunks.push(TextChunk::Text(current_text)); }
    chunks
}

fn process_text_chunk(text: &str) -> String {
    if !has_rtl_chars(text) { return text.to_string(); }
    let reshaped = reshape_line(text);
    if has_rtl_chars(&reshaped) { process_line(&reshaped) }
    else { reshaped }
}

pub fn process_buffer(text: &str) -> String {
    if !has_rtl_chars(text) { return text.to_string(); }
    let chunks = split_ansi_sequences(text);
    let mut result = String::with_capacity(text.len() * 2);
    for chunk in chunks {
        match chunk {
            TextChunk::AnsiSequence(seq) => { result.push_str(&seq); }
            TextChunk::Text(text) => { result.push_str(&process_text_chunk(&text)); }
        }
    }
    result
}

pub struct StreamProcessor {
    utf8_buffer: Vec<u8>,
    line_buffer: String,
    in_ansi_sequence: bool,
    ansi_buffer: String,
}

impl StreamProcessor {
    pub fn new() -> Self {
        StreamProcessor {
            utf8_buffer: Vec::new(),
            line_buffer: String::new(),
            in_ansi_sequence: false,
            ansi_buffer: String::new(),
        }
    }

    pub fn process_bytes(&mut self, data: &[u8]) -> String {
        let mut all_bytes = Vec::new();
        all_bytes.extend_from_slice(&self.utf8_buffer);
        all_bytes.extend_from_slice(data);
        self.utf8_buffer.clear();
        match std::str::from_utf8(&all_bytes) {
            Ok(text) => self.process_text(text),
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                let remaining = all_bytes.len() - valid_up_to;
                if remaining <= 4 && valid_up_to > 0 {
                    let valid_text = std::str::from_utf8(&all_bytes[..valid_up_to]).unwrap();
                    self.utf8_buffer = all_bytes[valid_up_to..].to_vec();
                    self.process_text(valid_text)
                } else {
                    let valid_text = std::str::from_utf8(&all_bytes[..valid_up_to]).unwrap();
                    self.process_text(valid_text)
                }
            }
        }
    }

    fn process_text(&mut self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for ch in text.chars() {
            if ch == '\n' || ch == '\r' {
                if !self.line_buffer.is_empty() {
                    result.push_str(&process_buffer(&self.line_buffer));
                    self.line_buffer.clear();
                }
                result.push(ch);
            } else if self.in_ansi_sequence {
                self.ansi_buffer.push(ch);
                if ch.is_ascii_alphabetic() {
                    result.push_str(&self.ansi_buffer);
                    self.ansi_buffer.clear();
                    self.in_ansi_sequence = false;
                }
            } else if ch == '\x1b' {
                if !self.line_buffer.is_empty() {
                    result.push_str(&process_buffer(&self.line_buffer));
                    self.line_buffer.clear();
                }
                self.in_ansi_sequence = true;
                self.ansi_buffer.push(ch);
            } else {
                self.line_buffer.push(ch);
            }
        }
        result
    }

    pub fn flush(&mut self) -> String {
        let mut result = String::new();
        if !self.line_buffer.is_empty() {
            result.push_str(&process_buffer(&self.line_buffer));
            self.line_buffer.clear();
        }
        if !self.ansi_buffer.is_empty() {
            result.push_str(&self.ansi_buffer);
            self.ansi_buffer.clear();
            self.in_ansi_sequence = false;
        }
        result
    }
}
