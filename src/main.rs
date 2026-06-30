mod pty;
mod bidi;
mod reshaper;
mod text_processor;

use std::env;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, BorrowedFd};
use std::process;
use pty::{PtySession, WinSize, get_terminal_size, is_tty};
use text_processor::StreamProcessor;
use nix::sys::wait::{waitpid, WaitStatus, WNOHANG};

const BUFFER_SIZE: usize = 4096;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => { print_help(); process::exit(0); }
            "--version" | "-V" => { println!("farsi-shell {}", VERSION); process::exit(0); }
            _ => {}
        }
    }

    if !is_tty() {
        eprintln!("farsi-shell: stdin is not a terminal");
        process::exit(1);
    }

    let win_size = get_terminal_size().unwrap_or(WinSize { rows: 24, cols: 80 });
    let shell = if args.len() > 1 && !args[1].starts_with('-') {
        Some(args[1].as_str())
    } else {
        None
    };

    eprintln!("farsi-shell v{} - Persian/Arabic text display for Termux", VERSION);
    eprintln!("Type 'exit' or press Ctrl+D to exit");
    eprintln!();

    let session = match PtySession::new(shell, win_size) {
        Ok(session) => session,
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
    };

    match run_loop(&session) {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
    }
}

fn run_loop(session: &PtySession) -> io::Result<i32> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut processor = StreamProcessor::new();
    let mut input_buf = [0u8; BUFFER_SIZE];
    let mut output_buf = [0u8; BUFFER_SIZE];
    let master_fd = session.master_fd();
    let stdin_fd = stdin.as_raw_fd();

    loop {
        if !session.is_alive() {
            while session.has_data(0)? {
                let n = session.read(&mut output_buf)?;
                if n == 0 { break; }
                let processed = processor.process_bytes(&output_buf[..n]);
                stdout.write_all(processed.as_bytes())?;
            }
            stdout.flush()?;
            match waitpid(session.child_pid(), Some(WNOHANG)) {
                Ok(WaitStatus::Exited(_, code)) => return Ok(code),
                Ok(WaitStatus::Signaled(_, signal, _)) => return Ok(128 + signal as i32),
                _ => return Ok(0),
            }
        }

        let mut readfds = nix::sys::select::FdSet::new();
        let stdin_borrowed = unsafe { BorrowedFd::borrow_raw(stdin_fd) };
        let master_borrowed = unsafe { BorrowedFd::borrow_raw(master_fd) };
        readfds.insert(&stdin_borrowed);
        readfds.insert(&master_borrowed);
        let mut timeout = nix::sys::time::TimeVal::new(1, 0);

        let result = nix::sys::select::select(
            Some(std::cmp::max(stdin_fd, master_fd) + 1),
            &mut readfds, None, None, &mut timeout,
        )?;

        if result == 0 { continue; }

        if readfds.contains(&stdin_borrowed) {
            let n = stdin.lock().read(&mut input_buf)?;
            if n == 0 { break; }
            session.write(&input_buf[..n])?;
        }

        if readfds.contains(&master_borrowed) {
            let n = session.read(&mut output_buf)?;
            if n == 0 { break; }
            let processed = processor.process_bytes(&output_buf[..n]);
            stdout.write_all(processed.as_bytes())?;
            stdout.flush()?;
        }
    }

    let remaining = processor.flush();
    if !remaining.is_empty() {
        stdout.write_all(remaining.as_bytes())?;
        stdout.flush()?;
    }
    Ok(0)
}

fn print_help() {
    println!("farsi-shell v{} - Persian/Arabic text display for Termux", VERSION);
    println!();
    println!("USAGE: farsi-shell [OPTIONS] [SHELL]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help");
    println!("    -V, --version    Print version");
    println!();
    println!("EXAMPLES:");
    println!("    farsi-shell");
    println!("    farsi-shell /data/data/com.termux/files/usr/bin/zsh");
}
