use std::io;
use std::os::unix::io::{AsRawFd, RawFd, AsFd, BorrowedFd};
use nix::pty::{openpty, OpenptyResult};
use nix::unistd::{fork, ForkResult, setsid, dup2, close, Pid};
use nix::sys::termios::{self, Termios, SetArg, LocalFlags, InputFlags, OutputFlags};
use nix::sys::select::FdSet;
use nix::sys::time::TimeVal;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub struct WinSize {
    pub rows: u16,
    pub cols: u16,
}

pub struct PtySession {
    master_fd: RawFd,
    child_pid: Pid,
    original_termios: Option<Termios>,
}

impl PtySession {
    pub fn new(shell: Option<&str>, win_size: WinSize) -> io::Result<Self> {
        let shell_path = shell
            .map(|s| s.to_string())
            .or_else(|| std::env::var("SHELL").ok())
            .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/bash".to_string());

        let OpenptyResult { master, slave } = openpty(None, None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("openpty failed: {}", e)))?;

        let master_fd = master.as_raw_fd();
        let slave_fd = slave.as_raw_fd();

        set_pty_size(master_fd, win_size.rows, win_size.cols)?;

        let stdin_fd = std::io::stdin().as_raw_fd();
        let original_termios = termios::tcgetattr(unsafe { BorrowedFd::borrow_raw(stdin_fd) }).ok();

        unsafe {
            match fork().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("fork failed: {}", e)))? {
                ForkResult::Parent { child } => {
                    close(slave_fd).ok();
                    if let Ok(mut t) = termios::tcgetattr(unsafe { BorrowedFd::borrow_raw(stdin_fd) }) {
                        set_raw_mode(&mut t);
                        let _ = termios::tcsetattr(unsafe { BorrowedFd::borrow_raw(stdin_fd) }, SetArg::TCSANOW, &t);
                    }
                    Ok(PtySession { master_fd, child_pid: child, original_termios })
                }
                ForkResult::Child => {
                    close(master_fd).ok();
                    setsid().ok();
                    #[cfg(target_os = "linux")]
                    { libc::ioctl(slave_fd, 0x540E, 0); }
                    let _ = dup2(slave_fd, 0);
                    let _ = dup2(slave_fd, 1);
                    let _ = dup2(slave_fd, 2);
                    if slave_fd > 2 { close(slave_fd).ok(); }
                    std::env::set_var("TERM", "xterm-256color");
                    std::env::set_var("FARSI_SHELL", "1");
                    let shell_cstr = CString::new(shell_path).expect("null byte");
                    let _ = nix::unistd::execvp(shell_cstr.as_c_str(), &[shell_cstr.as_c_str()]);
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn master_fd(&self) -> RawFd { self.master_fd }
    pub fn child_pid(&self) -> Pid { self.child_pid }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        nix::unistd::read(self.master_fd, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("read failed: {}", e)))
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        nix::unistd::write(self.master_fd, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("write failed: {}", e)))
    }

    pub fn has_data(&self, timeout_ms: u64) -> io::Result<bool> {
        let mut readfds = FdSet::new();
        let master_borrowed = unsafe { BorrowedFd::borrow_raw(self.master_fd) };
        readfds.insert(&master_borrowed);
        let mut timeout = TimeVal::new(
            (timeout_ms / 1000) as i64,
            ((timeout_ms % 1000) * 1000) as i64,
        );
        let result = nix::sys::select::select(
            Some(self.master_fd + 1),
            &mut readfds, None, None, &mut timeout,
        ).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("select failed: {}", e)))?;
        Ok(result > 0 && readfds.contains(&master_borrowed))
    }

    pub fn is_alive(&self) -> bool {
        use nix::sys::wait::{waitpid, WaitStatus};
        match waitpid(self.child_pid, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::StillAlive) => true,
            Ok(_) => false,
            Err(_) => true,
        }
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        if let Some(ref t) = self.original_termios {
            let stdin_fd = std::io::stdin().as_raw_fd();
            let _ = termios::tcsetattr(unsafe { BorrowedFd::borrow_raw(stdin_fd) }, SetArg::TCSANOW, t);
        }
    }
}

fn set_pty_size(fd: RawFd, rows: u16, cols: u16) -> io::Result<()> {
    let winsize = libc::winsize { ws_row: rows, ws_col: cols, ws_xpixel: 0, ws_ypixel: 0 };
    let result = unsafe { libc::ioctl(fd, libc::TIOCSWINSZ, &winsize) };
    if result < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}

fn set_raw_mode(termios: &mut Termios) {
    termios.local_flags.remove(LocalFlags::ICANON);
    termios.local_flags.remove(LocalFlags::ECHO);
    termios.local_flags.remove(LocalFlags::ISIG);
    termios.input_flags.remove(InputFlags::IXON);
    termios.input_flags.remove(InputFlags::ICRNL);
    termios.output_flags.remove(OutputFlags::OPOST);
}

pub fn is_tty() -> bool { unsafe { libc::isatty(0) != 0 } }

pub fn get_terminal_size() -> io::Result<WinSize> {
    let mut winsize = libc::winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let result = unsafe { libc::ioctl(0, libc::TIOCGWINSZ, &mut winsize) };
    if result < 0 { Ok(WinSize { rows: 24, cols: 80 }) }
    else { Ok(WinSize { rows: winsize.ws_row, cols: winsize.ws_col }) }
}
