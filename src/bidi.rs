/// Bidirectional Text Algorithm (simplified for terminal use)

use crate::reshaper::{is_arabic_letter, is_diacritic};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    LTR,
    RTL,
    Neutral,
}

pub fn char_direction(ch: char) -> Direction {
    let cp = ch as u32;

    if is_arabic_letter(ch) || is_diacritic(ch) {
        return Direction::RTL;
    }

    if (0xFB50..=0xFDFF).contains(&cp) {
        return Direction::RTL;
    }

    if (0xFE70..=0xFEFF).contains(&cp) {
        return Direction::RTL;
    }

    if (0x0590..=0x05FF).contains(&cp) {
        return Direction::RTL;
    }

    if ch.is_whitespace() || ch.is_ascii_punctuation() {
        return Direction::Neutral;
    }

    Direction::LTR
}

pub fn has_rtl_chars(text: &str) -> bool {
    text.chars().any(|c| char_direction(c) == Direction::RTL)
}

#[allow(dead_code)]
pub fn is_rtl_dominant(text: &str) -> bool {
    let mut rtl_count = 0;
    let mut ltr_count = 0;

    for ch in text.chars() {
        match char_direction(ch) {
            Direction::RTL => rtl_count += 1,
            Direction::LTR => ltr_count += 1,
            _ => {}
        }
    }

    rtl_count > ltr_count
}

pub fn reorder_line(text: &str) -> String {
    if !has_rtl_chars(text) {
        return text.to_string();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len() * 2);
    let mut i = 0;

    while i < chars.len() {
        let dir = char_direction(chars[i]);

        if dir == Direction::RTL {
            let start = i;
            while i < chars.len() {
                let d = char_direction(chars[i]);
                if d == Direction::RTL {
                    i += 1;
                } else if d == Direction::Neutral && i + 1 < chars.len() && char_direction(chars[i + 1]) == Direction::RTL {
                    i += 1;
                } else {
                    break;
                }
            }

            for ch in chars[start..i].iter().rev() {
                result.push(*ch);
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

pub fn process_line(reshaped: &str) -> String {
    if !has_rtl_chars(reshaped) {
        return reshaped.to_string();
    }

    reorder_line(reshaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_direction() {
        assert_eq!(char_direction('A'), Direction::LTR);
        assert_eq!(char_direction('ا'), Direction::RTL);
        assert_eq!(char_direction(' '), Direction::Neutral);
        assert_eq!(char_direction('1'), Direction::LTR);
    }

    #[test]
    fn test_has_rtl_chars() {
        assert!(has_rtl_chars("Hello سلام"));
        assert!(!has_rtl_chars("Hello World"));
        assert!(has_rtl_chars("سلام"));
    }

    #[test]
    fn test_is_rtl_dominant() {
        assert!(is_rtl_dominant("سلام دنیا"));
        assert!(!is_rtl_dominant("Hello World"));
    }

    #[test]
    fn test_presentation_forms_detected() {
        assert_eq!(char_direction('\u{FB56}'), Direction::RTL);
        assert_eq!(char_direction('\u{FB7A}'), Direction::RTL);
        assert_eq!(char_direction('\u{FB8E}'), Direction::RTL);
    }

    #[test]
    fn test_reorder_line() {
        let input = "سلام";
        let reordered = reorder_line(input);
        let input_chars: Vec<char> = input.chars().collect();
        let reordered_chars: Vec<char> = reordered.chars().collect();
        assert_eq!(reordered_chars[0], input_chars[input_chars.len() - 1]);
    }

    #[test]
    fn test_reorder_mixed() {
        let input = "Hello سلام World";
        let reordered = reorder_line(input);
        assert!(reordered.starts_with("Hello "));
        assert!(reordered.ends_with(" World"));
    }
}
