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
use nix::sys::wait::{waitpid, WaitStatus, WaitPidFlag};

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
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut processor = StreamProcessor::new();
    let mut input_buf = [0u8; BUFFER_SIZE];
    let mut output_buf = [0u8; BUFFER_SIZE];
    let master_fd = session.master_fd();
    let stdin_fd = stdin.as_raw_fd();

    loop {
        if !session.is_alive() {
            while session.has_data(0).unwrap_or(false) {
                let n = session.read(&mut output_buf).unwrap_or(0);
                if n == 0 { break; }
                let processed = processor.process_bytes(&output_buf[..n]);
                let _ = stdout.write_all(processed.as_bytes());
            }
            let _ = stdout.flush();
            match waitpid(session.child_pid(), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::Exited(_, code)) => return Ok(code),
                Ok(WaitStatus::Signaled(_, signal, _)) => return Ok(128 + signal as i32),
                _ => return Ok(0),
            }
        }

        // Simple sleep to reduce CPU usage
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Read from master (shell output)
        if let Ok(n) = session.read(&mut output_buf) {
            if n > 0 {
                let processed = processor.process_bytes(&output_buf[..n]);
                let _ = stdout.write_all(processed.as_bytes());
                let _ = stdout.flush();
            }
        }

        // Read from stdin (user input)
        if let Ok(n) = stdin.read(&mut input_buf) {
            if n > 0 {
                let _ = session.write(&input_buf[..n]);
            } else {
                break;
            }
        }
    }

    let remaining = processor.flush();
    if !remaining.is_empty() {
        let _ = stdout.write_all(remaining.as_bytes());
    }
    Ok(0)
}

fn print_help() {
    println!("farsi-shell v{} - Persian/Arabic text display for Termux", VERSION);
    println!();
    println!("USAGE: farsi-shell [SHELL]");
    println!();
    println!("Example: farsi-shell");
}
