a/farsi-shell\src\reshaper.rs → b/farsi-shell\src\reshaper.rs
@@ -399,8 +399,18 @@
                 result.push(get_shaped_char(ch, true, false, false, false));
             }
             JoiningType::Right => {
+                // Right-joining connects to previous if it's Dual-joining
                 let connects_prev = if idx > 0 {
-                    matches!(joining[idx - 1], JoiningType::Dual | JoiningType::Right)
+                    // Look back past transparent characters
+                    let mut j = idx - 1;
+                    loop {
+                        if joining[j] == JoiningType::Transparent {
+                            if j == 0 { break false; }
+                            j -= 1;
+                        } else {
+                            break matches!(joining[j], JoiningType::Dual);
+                        }
+                    }
                 } else {
                     false
                 };
@@ -412,6 +422,7 @@
                 }
             }
             JoiningType::Dual => {
+                // Check if previous non-transparent character is Dual-joining
                 let connects_prev = if idx > 0 {
                     let mut j = idx - 1;
                     loop {
@@ -426,6 +437,7 @@
                     false
                 };
 
+                // Check if next non-transparent character is Dual or Right-joining
                 let connects_next = if idx < len - 1 {
                     let mut j = idx + 1;
                     loop {
@@ -478,7 +490,10 @@
         let input = "سلام";
         let shaped = reshape_line(input);
         // Should produce presentation forms
-        assert!(shaped.chars().all(|c| (c as u32 >= 0xFE70 && c as u32 <= 0xFEFF) || !is_arabic_letter(c)));
+        assert!(shaped.chars().all(|c| {
+            let cp = c as u32;
+            (0xFE70..=0xFEFF).contains(&cp) || !is_arabic_letter(c)
+        }));
     }
 
     #[test]
@@ -488,4 +503,23 @@
         assert!(shaped.starts_with("Hello "));
         assert!(shaped.ends_with(" World"));
     }
-}
+
+    #[test]
+    fn test_reshape_word_connectivity() {
+        // بسم الله - test that letters connect properly
+        let input = "بسم";
+        let shaped = reshape_line(input);
+        // First letter ب should be initial, س medial, م final
+        let chars: Vec<char> = shaped.chars().collect();
+        assert_eq!(chars[0], '\u{FE91}'); // ب initial
+        assert_eq!(chars[1], '\u{FEB4}'); // س medial
+        assert_eq!(chars[2], '\u{FEE2}'); // م final
+    }
+
+    #[test]
+    fn test_diacritics_preserved() {
+        let input = "بَ";
+        let shaped = reshape_line(input);
+        assert!(shaped.contains('\u{064B}')); // Fathatan preserved
+    }
+}
