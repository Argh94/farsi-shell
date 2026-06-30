mod pty;
mod bidi;
mod reshaper;
mod text_processor;

use std::env;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
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
    eprintln!("Type 'exit' or press Ctrl+D to exit\n");

    let session = match PtySession::new(shell, win_size) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error creating PTY: {}", e); process::exit(1); }
    };

    if let Err(e) = run_loop(&session) {
        eprintln!("Error: {}", e);
    }
}

fn run_loop(session: &PtySession) -> io::Result<()> {
    let mut processor = StreamProcessor::new();
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        // Read from shell output
        match session.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let processed = processor.process_bytes(&buf[..n]);
                let _ = io::stdout().write_all(processed.as_bytes());
                let _ = io::stdout().flush();
            }
            Err(_) => {}
        }

        // Read user input (non-blocking attempt)
        if let Ok(n) = io::stdin().read(&mut buf) {
            if n == 0 {
                break;
            }
            let _ = session.write(&buf[..n]);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        if !session.is_alive() {
            break;
        }
    }
    Ok(())
}

fn print_help() {
    println!("farsi-shell - Persian text fixer for Termux");
}
