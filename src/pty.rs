a/farsi-shell\src\pty.rs → b/farsi-shell\src\pty.rs
@@ -6,7 +6,7 @@
 /// - Forwarding input/output between the user's terminal and the shell
 /// - Setting terminal size
 
-use std::io::{self, Read, Write};
+use std::io;
 use std::os::unix::io::{AsRawFd, RawFd};
 use nix::pty::{openpty, OpenptyResult};
 use nix::unistd::{fork, ForkResult, setsid, dup2, close, Pid};
@@ -78,9 +78,9 @@
                     close(master_fd).ok();
 
                     // Create a new session
-                    setsid().map_err(|e| e).ok();
-
-                    // Set slave as controlling terminal (Linux-specific)
+                    setsid().ok();
+
+                    // Set slave as controlling terminal (Linux/Android)
                     #[cfg(target_os = "linux")]
                     {
                         const TIOCSCTTY: libc::c_ulong = 0x540E;
@@ -88,9 +88,9 @@
                     }
 
                     // Redirect stdin/stdout/stderr to slave
-                    dup2(slave_fd, 0).ok(); // stdin
-                    dup2(slave_fd, 1).ok(); // stdout
-                    dup2(slave_fd, 2).ok(); // stderr
+                    let _ = dup2(slave_fd, 0); // stdin
+                    let _ = dup2(slave_fd, 1); // stdout
+                    let _ = dup2(slave_fd, 2); // stderr
 
                     // Close original slave fd if it's not one of the standard fds
                     if slave_fd > 2 {
@@ -102,14 +102,16 @@
                     std::env::set_var("FARSI_SHELL", "1");
 
                     // Execute the shell
-                    let shell_cstr = CString::new(shell_path.clone())
-                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
-
-                    // Use execvp from nix
-                    nix::unistd::execvp(&shell_cstr, &[shell_cstr.clone()])
-                        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("exec failed: {}", e)))?;
-
-                    // Should not reach here
+                    let shell_cstr = CString::new(shell_path)
+                        .expect("shell path contains null byte");
+
+                    // execvp takes &CStr and &[&CStr]
+                    let _ = nix::unistd::execvp(
+                        shell_cstr.as_c_str(),
+                        &[shell_cstr.as_c_str()],
+                    );
+
+                    // execvp only returns on error
                     std::process::exit(1);
                 }
             }
@@ -161,6 +163,7 @@
     }
 
     /// Resize the PTY
+    #[allow(dead_code)]
     pub fn resize(&self, rows: u16, cols: u16) -> io::Result<()> {
         set_pty_size(self.master_fd, rows, cols)
     }
