use std::io;
use std::os::unix::io::RawFd;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub struct WinSize {
    pub rows: u16,
    pub cols: u16,
}

pub struct PtySession {
    master_fd: RawFd,
    child_pid: i32,
}

impl PtySession {
    pub fn new(shell: Option<&str>, win_size: WinSize) -> io::Result<Self> {
        let shell_path = shell
            .map(|s| s.to_string())
            .or_else(|| std::env::var("SHELL").ok())
            .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/bash".to_string());

        // Open PTY using libc
        let mut master_fd: RawFd = 0;
        let slave_fd = unsafe { libc::openpty(&mut master_fd, std::ptr::null_mut(), std::ptr::null(), std::ptr::null(), std::ptr::null()) };
        if slave_fd < 0 {
            return Err(io::Error::last_os_error());
        }

        // Get the slave fd from openpty
        // Actually, openpty puts the slave fd in the second parameter
        // Let me use the correct API

        // Create PTY pair
        let mut master: RawFd = 0;
        let mut slave: RawFd = 0;
        let ret = unsafe { libc::openpty(&mut master, &mut slave, std::ptr::null_mut(), std::ptr::null(), std::ptr::null()) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        // Set terminal size
        set_pty_size(master, win_size.rows, win_size.cols)?;

        // Fork
        let pid = unsafe { libc::fork() };
        if pid < 0 {
            unsafe { libc::close(master); }
            unsafe { libc::close(slave); }
            return Err(io::Error::last_os_error());
        }

        if pid == 0 {
            // Child process
            unsafe { libc::close(master); }

            // Create new session
            unsafe { libc::setsid(); }

            // Set slave as controlling terminal
            unsafe { libc::ioctl(slave, libc::TIOCSCTTY, 0); }

            // Redirect stdin/stdout/stderr
            unsafe { libc::dup2(slave, 0); }
            unsafe { libc::dup2(slave, 1); }
            unsafe { libc::dup2(slave, 2); }

            if slave > 2 {
                unsafe { libc::close(slave); }
            }

            // Set environment
            std::env::set_var("TERM", "xterm-256color");
            std::env::set_var("FARSI_SHELL", "1");

            // Execute shell
            let shell_cstr = CString::new(shell_path).expect("null byte");
            let shell_ptr = shell_cstr.as_ptr();
            let args = [shell_ptr, std::ptr::null()];
            unsafe { libc::execvp(shell_ptr, args.as_ptr()); }

            // execvp only returns on error
            unsafe { libc::_exit(1); }
        }

        // Parent process
        unsafe { libc::close(slave); }

        Ok(PtySession {
            master_fd: master,
            child_pid: pid,
        })
    }

    pub fn master_fd(&self) -> RawFd {
        self.master_fd
    }

    pub fn child_pid(&self) -> i32 {
        self.child_pid
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let n = unsafe {
            libc::read(
                self.master_fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len(),
            )
        };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let n = unsafe {
            libc::write(
                self.master_fd,
                buf.as_ptr() as *const libc::c_void,
                buf.len(),
            )
        };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }

    pub fn has_data(&self, timeout_ms: u64) -> io::Result<bool> {
        let mut readfds: libc::fd_set = unsafe { std::mem::zeroed() };
        unsafe { libc::FD_ZERO(&mut readfds); }
        unsafe { libc::FD_SET(self.master_fd, &mut readfds); }

        let mut timeout = libc::timeval {
            tv_sec: (timeout_ms / 1000) as i64,
            tv_usec: ((timeout_ms % 1000) * 1000) as i64,
        };

        let result = unsafe {
            libc::select(
                self.master_fd + 1,
                &mut readfds,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut timeout,
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(result > 0 && unsafe { libc::FD_ISSET(self.master_fd, &readfds) })
    }

    pub fn is_alive(&self) -> bool {
        let mut status: i32 = 0;
        let result = unsafe { libc::waitpid(self.child_pid, &mut status, libc::WNOHANG) };
        result == 0
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        unsafe { libc::close(self.master_fd); }
    }
}

fn set_pty_size(fd: RawFd, rows: u16, cols: u16) -> io::Result<()> {
    let winsize = libc::winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let result = unsafe { libc::ioctl(fd, libc::TIOCSWINSZ, &winsize) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

pub fn get_terminal_size() -> io::Result<WinSize> {
    let mut winsize: libc::winsize = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::ioctl(libc::STDIN_FILENO, libc::TIOCGWINSZ, &mut winsize) };
    if result < 0 {
        Ok(WinSize { rows: 24, cols: 80 })
    } else {
        Ok(WinSize {
            rows: winsize.ws_row,
            cols: winsize.ws_col,
        })
    }
}
