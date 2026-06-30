use crate::reshaper::{is_arabic_letter, is_diacritic};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction { LTR, RTL, Neutral }

pub fn char_direction(ch: char) -> Direction {
    let cp = ch as u32;
    if is_arabic_letter(ch) || is_diacritic(ch) { return Direction::RTL; }
    if (0xFB50..=0xFDFF).contains(&cp) { return Direction::RTL; }
    if (0xFE70..=0xFEFF).contains(&cp) { return Direction::RTL; }
    if (0x0590..=0x05FF).contains(&cp) { return Direction::RTL; }
    if ch.is_whitespace() || ch.is_ascii_punctuation() { return Direction::Neutral; }
    Direction::LTR
}

pub fn has_rtl_chars(text: &str) -> bool {
    text.chars().any(|c| char_direction(c) == Direction::RTL)
}

pub fn reorder_line(text: &str) -> String {
    if !has_rtl_chars(text) { return text.to_string(); }
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len() * 2);
    let mut i = 0;
    while i < chars.len() {
        let dir = char_direction(chars[i]);
        if dir == Direction::RTL {
            let start = i;
            while i < chars.len() {
                let d = char_direction(chars[i]);
                if d == Direction::RTL { i += 1; }
                else if d == Direction::Neutral && i + 1 < chars.len() && char_direction(chars[i + 1]) == Direction::RTL { i += 1; }
                else { break; }
            }
            for ch in chars[start..i].iter().rev() { result.push(*ch); }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

pub fn process_line(reshaped: &str) -> String {
    if !has_rtl_chars(reshaped) { return reshaped.to_string(); }
    reorder_line(reshaped)
}
