a/farsi-shell\src\main.rs → b/farsi-shell\src\main.rs
@@ -32,7 +32,6 @@
 use std::process;
 use pty::{PtySession, WinSize, get_terminal_size, is_tty};
 use text_processor::StreamProcessor;
-use nix::unistd::Pid;
 use nix::sys::wait::{waitpid, WaitStatus, WNOHANG};
 
 /// Buffer size for reading/writing
@@ -70,8 +69,8 @@
     // Get terminal size
     let win_size = get_terminal_size().unwrap_or(WinSize { rows: 24, cols: 80 });
 
-    // Determine shell to use
-    let shell = if args.len() > 1 {
+    // Determine shell to use (skip if args[1] is a flag)
+    let shell = if args.len() > 1 && !args[1].starts_with('-') {
         Some(args[1].as_str())
     } else {
         None
@@ -113,7 +112,7 @@
 
 /// Main I/O loop
 fn run_loop(session: &PtySession) -> io::Result<i32> {
-    let mut stdin = io::stdin();
+    let stdin = io::stdin();
     let mut stdout = io::stdout();
     let mut processor = StreamProcessor::new();
 
@@ -167,7 +166,7 @@
 
         // Check if there's keyboard input
         if readfds.contains(stdin_fd) {
-            let n = stdin.read(&mut input_buf)?;
+            let n = stdin.lock().read(&mut input_buf)?;
             if n == 0 {
                 // EOF on stdin (Ctrl+D)
                 break;
@@ -240,16 +239,6 @@
     println!("    # Auto-start in .bashrc");
     println!("    echo 'farsi-shell' >> ~/.bashrc");
     println!();
-    println!("INSTALLATION:");
-    println!("    # Build for Termux");
-    println!("    cargo build --release --target aarch64-linux-android");
-    println!();
-    println!("    # Copy to Termux");
-    println!("    cp target/aarch64-linux-android/release/farsi-shell $PREFIX/bin/");
-    println!();
-    println!("    # Or use the install script");
-    println!("    curl -sSL https://raw.githubusercontent.com/youruser/farsi-shell/main/install.sh | bash");
-    println!();
     println!("AUTHOR:");
     println!("    Written for the Persian-speaking Termux community.");
     println!();
