/// Arabic/Persian character reshaping (joining letter forms)
///
/// Arabic script letters have up to 4 forms:
///   - Isolated: when standalone
///   - Initial: when at the beginning of a word (connects to next)
///   - Medial: when in the middle (connects to both sides)
///   - Final: when at the end (connects to previous)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoiningType {
    Dual,
    Right,
    None,
    Transparent,
}

pub fn is_arabic_letter(ch: char) -> bool {
    let cp = ch as u32;
    matches!(cp,
        0x0621..=0x063A |
        0x0641..=0x064A |
        0x067E |
        0x0686 |
        0x0698 |
        0x06A9 |
        0x06AF |
        0x06CC |
        0x0671 |
        0x0679..=0x067D |
        0x0680..=0x069C |
        0x06A0..=0x06B9 |
        0x06BB..=0x06D3 |
        0x06D5 |
        0x06EE..=0x06EF |
        0x06F0..=0x06F9 |
        0x0750..=0x077F |
        0x08A0..=0x08FF
    )
}

pub fn is_diacritic(ch: char) -> bool {
    let cp = ch as u32;
    matches!(cp,
        0x0610..=0x061A |
        0x064B..=0x065F |
        0x0670 |
        0x06D6..=0x06DC |
        0x06DF..=0x06E4 |
        0x06E7..=0x06E8 |
        0x06EA..=0x06ED |
        0x08D4..=0x08E1 |
        0x08F0..=0x08FF
    )
}

pub fn joining_type(ch: char) -> JoiningType {
    if is_diacritic(ch) {
        return JoiningType::Transparent;
    }

    match ch {
        '\u{0621}' => JoiningType::None,

        '\u{0622}' |
        '\u{0623}' |
        '\u{0624}' |
        '\u{0625}' |
        '\u{0627}' |
        '\u{0629}' |
        '\u{062F}' |
        '\u{0630}' |
        '\u{0631}' |
        '\u{0632}' |
        '\u{0648}' |
        '\u{0671}' |
        '\u{067E}' |
        '\u{0698}' |
        '\u{06CC}' => JoiningType::Right,

        ch if is_arabic_letter(ch) => JoiningType::Dual,

        _ => JoiningType::None,
    }
}

pub fn get_shaped_char(ch: char, isolated: bool, initial: bool, _medial: bool, final_pos: bool) -> char {
    match ch {
        '\u{0623}' => if isolated { '\u{FE83}' } else { '\u{FE84}' },
        '\u{0622}' => if isolated { '\u{FE81}' } else { '\u{FE82}' },
        '\u{0625}' => if isolated { '\u{FE87}' } else { '\u{FE88}' },
        '\u{0627}' => if isolated { '\u{FE8D}' } else { '\u{FE8E}' },
        '\u{0628}' => {
            if isolated { '\u{FE8F}' }
            else if initial { '\u{FE91}' }
            else if final_pos { '\u{FE90}' }
            else { '\u{FE92}' }
        }
        '\u{0629}' => if isolated { '\u{FE93}' } else { '\u{FE94}' },
        '\u{062A}' => {
            if isolated { '\u{FE95}' }
            else if initial { '\u{FE97}' }
            else if final_pos { '\u{FE96}' }
            else { '\u{FE98}' }
        }
        '\u{062B}' => {
            if isolated { '\u{FE99}' }
            else if initial { '\u{FE9B}' }
            else if final_pos { '\u{FE9A}' }
            else { '\u{FE9C}' }
        }
        '\u{062C}' => {
            if isolated { '\u{FE9D}' }
            else if initial { '\u{FE9F}' }
            else if final_pos { '\u{FE9E}' }
            else { '\u{FEA0}' }
        }
        '\u{062D}' => {
            if isolated { '\u{FEA1}' }
            else if initial { '\u{FEA3}' }
            else if final_pos { '\u{FEA2}' }
            else { '\u{FEA4}' }
        }
        '\u{062E}' => {
            if isolated { '\u{FEA5}' }
            else if initial { '\u{FEA7}' }
            else if final_pos { '\u{FEA6}' }
            else { '\u{FEA8}' }
        }
        '\u{062F}' => if isolated { '\u{FEA9}' } else { '\u{FEAA}' },
        '\u{0630}' => if isolated { '\u{FEAB}' } else { '\u{FEAC}' },
        '\u{0631}' => if isolated { '\u{FEAD}' } else { '\u{FEAE}' },
        '\u{0632}' => if isolated { '\u{FEAF}' } else { '\u{FEB0}' },
        '\u{0633}' => {
            if isolated { '\u{FEB1}' }
            else if initial { '\u{FEB3}' }
            else if final_pos { '\u{FEB2}' }
            else { '\u{FEB4}' }
        }
        '\u{0634}' => {
            if isolated { '\u{FEB5}' }
            else if initial { '\u{FEB7}' }
            else if final_pos { '\u{FEB6}' }
            else { '\u{FEB8}' }
        }
        '\u{0635}' => {
            if isolated { '\u{FEB9}' }
            else if initial { '\u{FEBB}' }
            else if final_pos { '\u{FEBA}' }
            else { '\u{FEBC}' }
        }
        '\u{0636}' => {
            if isolated { '\u{FEBD}' }
            else if initial { '\u{FEBF}' }
            else if final_pos { '\u{FEBE}' }
            else { '\u{FEC0}' }
        }
        '\u{0637}' => {
            if isolated { '\u{FEC1}' }
            else if initial { '\u{FEC3}' }
            else if final_pos { '\u{FEC2}' }
            else { '\u{FEC4}' }
        }
        '\u{0638}' => {
            if isolated { '\u{FEC5}' }
            else if initial { '\u{FEC7}' }
            else if final_pos { '\u{FEC6}' }
            else { '\u{FEC8}' }
        }
        '\u{0639}' => {
            if isolated { '\u{FEC9}' }
            else if initial { '\u{FECB}' }
            else if final_pos { '\u{FECA}' }
            else { '\u{FECC}' }
        }
        '\u{063A}' => {
            if isolated { '\u{FECD}' }
            else if initial { '\u{FECF}' }
            else if final_pos { '\u{FECE}' }
            else { '\u{FED0}' }
        }
        '\u{0641}' => {
            if isolated { '\u{FED1}' }
            else if initial { '\u{FED3}' }
            else if final_pos { '\u{FED2}' }
            else { '\u{FED4}' }
        }
        '\u{0642}' => {
            if isolated { '\u{FED5}' }
            else if initial { '\u{FED7}' }
            else if final_pos { '\u{FED6}' }
            else { '\u{FED8}' }
        }
        '\u{0643}' => {
            if isolated { '\u{FED9}' }
            else if initial { '\u{FEDB}' }
            else if final_pos { '\u{FEDA}' }
            else { '\u{FEDC}' }
        }
        '\u{0644}' => {
            if isolated { '\u{FEDD}' }
            else if initial { '\u{FEDF}' }
            else if final_pos { '\u{FEDE}' }
            else { '\u{FEE0}' }
        }
        '\u{0645}' => {
            if isolated { '\u{FEE1}' }
            else if initial { '\u{FEE3}' }
            else if final_pos { '\u{FEE2}' }
            else { '\u{FEE4}' }
        }
        '\u{0646}' => {
            if isolated { '\u{FEE5}' }
            else if initial { '\u{FEE7}' }
            else if final_pos { '\u{FEE6}' }
            else { '\u{FEE8}' }
        }
        '\u{0647}' => {
            if isolated { '\u{FEE9}' }
            else if initial { '\u{FEEB}' }
            else if final_pos { '\u{FEEA}' }
            else { '\u{FEEC}' }
        }
        '\u{0648}' => if isolated { '\u{FEED}' } else { '\u{FEEE}' },
        '\u{064A}' => {
            if isolated { '\u{FEF1}' }
            else if initial { '\u{FEF3}' }
            else if final_pos { '\u{FEF2}' }
            else { '\u{FEF4}' }
        }
        '\u{0649}' => {
            if isolated { '\u{FEF0}' }
            else if initial { '\u{FEF3}' }
            else if final_pos { '\u{FEF2}' }
            else { '\u{FEF4}' }
        }
        '\u{0621}' => '\u{FE80}',

        // Persian characters
        '\u{067E}' => {
            if isolated { '\u{FB56}' }
            else if initial { '\u{FB58}' }
            else if final_pos { '\u{FB57}' }
            else { '\u{FB59}' }
        }
        '\u{0686}' => {
            if isolated { '\u{FB7A}' }
            else if initial { '\u{FB7C}' }
            else if final_pos { '\u{FB7B}' }
            else { '\u{FB7D}' }
        }
        '\u{0698}' => if isolated { '\u{FB8A}' } else { '\u{FB8B}' },
        '\u{06A9}' => {
            if isolated { '\u{FB8E}' }
            else if initial { '\u{FB90}' }
            else if final_pos { '\u{FB8F}' }
            else { '\u{FB91}' }
        }
        '\u{06AF}' => {
            if isolated { '\u{FB92}' }
            else if initial { '\u{FB94}' }
            else if final_pos { '\u{FB93}' }
            else { '\u{FB95}' }
        }
        '\u{06CC}' => {
            if isolated { '\u{FBFC}' }
            else if initial { '\u{FBFE}' }
            else if final_pos { '\u{FBFD}' }
            else { '\u{FBFF}' }
        }

        _ => ch,
    }
}

pub fn reshape_line(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len() * 2);
    let mut i = 0;

    while i < chars.len() {
        if !is_arabic_letter(chars[i]) && !is_diacritic(chars[i]) {
            result.push(chars[i]);
            i += 1;
            continue;
        }

        let mut word_chars: Vec<char> = Vec::new();
        let mut joining_info: Vec<JoiningType> = Vec::new();

        while i < chars.len() && (is_arabic_letter(chars[i]) || is_diacritic(chars[i])) {
            word_chars.push(chars[i]);
            joining_info.push(joining_type(chars[i]));
            i += 1;
        }

        let shaped = reshape_word(&word_chars, &joining_info);
        result.push_str(&shaped);
    }

    result
}

fn reshape_word(chars: &[char], joining: &[JoiningType]) -> String {
    let mut result = String::with_capacity(chars.len() * 2);
    let len = chars.len();

    for (idx, (&ch, &jt)) in chars.iter().zip(joining.iter()).enumerate() {
        match jt {
            JoiningType::Transparent => {
                result.push(ch);
            }
            JoiningType::None => {
                result.push(get_shaped_char(ch, true, false, false, false));
            }
            JoiningType::Right => {
                let connects_prev = if idx > 0 {
                    let mut j = idx - 1;
                    loop {
                        if joining[j] == JoiningType::Transparent {
                            if j == 0 { break false; }
                            j -= 1;
                        } else {
                            break matches!(joining[j], JoiningType::Dual);
                        }
                    }
                } else {
                    false
                };

                if connects_prev {
                    result.push(get_shaped_char(ch, false, false, false, true));
                } else {
                    result.push(get_shaped_char(ch, true, false, false, false));
                }
            }
            JoiningType::Dual => {
                let connects_prev = if idx > 0 {
                    let mut j = idx - 1;
                    loop {
                        if joining[j] == JoiningType::Transparent {
                            if j == 0 { break false; }
                            j -= 1;
                        } else {
                            break matches!(joining[j], JoiningType::Dual);
                        }
                    }
                } else {
                    false
                };

                let connects_next = if idx < len - 1 {
                    let mut j = idx + 1;
                    loop {
                        if j >= len { break false; }
                        if joining[j] == JoiningType::Transparent {
                            j += 1;
                        } else {
                            break matches!(joining[j], JoiningType::Dual | JoiningType::Right);
                        }
                    }
                } else {
                    false
                };

                match (connects_prev, connects_next) {
                    (true, true) => result.push(get_shaped_char(ch, false, false, true, false)),
                    (true, false) => result.push(get_shaped_char(ch, false, false, false, true)),
                    (false, true) => result.push(get_shaped_char(ch, false, true, false, false)),
                    (false, false) => result.push(get_shaped_char(ch, true, false, false, false)),
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joining_types() {
        assert_eq!(joining_type('ب'), JoiningType::Dual);
        assert_eq!(joining_type('ا'), JoiningType::Right);
        assert_eq!(joining_type('د'), JoiningType::Right);
        assert_eq!(joining_type('\u{064B}'), JoiningType::Transparent);
    }

    #[test]
    fn test_reshape_basic() {
        let input = "سلام";
        let shaped = reshape_line(input);
        assert!(shaped.chars().all(|c| {
            let cp = c as u32;
            (0xFE70..=0xFEFF).contains(&cp) || !is_arabic_letter(c)
        }));
    }

    #[test]
    fn test_mixed_text() {
        let input = "Hello سلام World";
        let shaped = reshape_line(input);
        assert!(shaped.starts_with("Hello "));
        assert!(shaped.ends_with(" World"));
    }

    #[test]
    fn test_reshape_word_connectivity() {
        let input = "بسم";
        let shaped = reshape_line(input);
        let chars: Vec<char> = shaped.chars().collect();
        assert_eq!(chars[0], '\u{FE91}');
        assert_eq!(chars[1], '\u{FEB4}');
        assert_eq!(chars[2], '\u{FEE2}');
    }

    #[test]
    fn test_diacritics_preserved() {
        let input = "بَ";
        let shaped = reshape_line(input);
        assert!(shaped.contains('\u{064B}'));
    }
}
