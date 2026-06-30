a/farsi-shell\src\bidi.rs → b/farsi-shell\src\bidi.rs
@@ -3,9 +3,6 @@
 /// This module handles the reordering of mixed LTR/RTL text for display.
 /// In a terminal, we need to reverse the visual order of RTL characters
 /// so they appear correctly on screen.
-///
-/// The Unicode BiDi algorithm (UBA) is complex. We implement a simplified
-/// version that handles the most common cases for terminal output.
 
 use crate::reshaper::{is_arabic_letter, is_diacritic};
 
@@ -15,7 +12,6 @@
     LTR,        // Left-to-Right (Latin, digits, etc.)
     RTL,        // Right-to-Left (Arabic, Hebrew)
     Neutral,    // Punctuation, spaces, etc.
-    EmbedRTL,   // RTL embedded in LTR context (number-like behavior)
 }
 
 /// Determine the direction of a character
@@ -42,17 +38,7 @@
         return Direction::Neutral;
     }
 
-    // Digits
-    if ch.is_ascii_digit() {
-        return Direction::LTR;
-    }
-
-    // Persian/Arabic digits
-    if (0x06F0..=0x06F9).contains(&cp) || (0x0660..=0x0669).contains(&cp) {
-        return Direction::RTL;
-    }
-
-    // Everything else (Latin, Cyrillic, CJK, etc.)
+    // Everything else (Latin, digits, etc.)
     Direction::LTR
 }
 
@@ -77,230 +63,54 @@
     rtl_count > ltr_count
 }
 
-/// A segment of text with uniform direction
-#[derive(Debug, Clone)]
-struct TextSegment {
-    text: String,
-    direction: Direction,
-}
-
-/// Split text into segments of uniform direction
-fn segment_text(text: &str) -> Vec<TextSegment> {
-    let mut segments: Vec<TextSegment> = Vec::new();
-    let mut current_text = String::new();
-    let mut current_dir: Option<Direction> = None;
-
-    for ch in text.chars() {
-        let dir = char_direction(ch);
-
-        // Determine if we need to start a new segment
-        let need_new_segment = match (current_dir, dir) {
-            (None, _) => false,  // First character
-            (Some(_), Direction::Neutral) => false,  // Neutrals join current segment
-            (Some(cur), d) if cur != d => true,  // Direction change
-            _ => false,
-        };
-
-        if need_new_segment {
-            // Save current segment
-            segments.push(TextSegment {
-                text: current_text.clone(),
-                direction: current_dir.unwrap_or(Direction::LTR),
-            });
-            current_text.clear();
-            current_dir = Some(dir);
-        } else if current_dir.is_none() {
-            current_dir = Some(dir);
-        }
-
-        current_text.push(ch);
-    }
… omitted 229 diff line(s) across 1 additional file(s)/section(s)
