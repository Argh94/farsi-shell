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

const BUFFER_SIZE: usize = 4096;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-V" => {
                println!("farsi-shell {}", VERSION);
                process::exit(0);
            }
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
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    match run_loop(&session) {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run_loop(session: &PtySession) -> io::Result<i32> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut processor = StreamProcessor::new();
    let mut input_buf = [0u8; BUFFER_SIZE];
    let mut output_buf = [0u8; BUFFER_SIZE];
    let master_fd = session.master_fd();

    loop {
        if !session.is_alive() {
            // Get remaining output
            loop {
                match session.has_data(0) {
                    Ok(true) => {
                        match session.read(&mut output_buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                let processed = processor.process_bytes(&output_buf[..n]);
                                let _ = stdout.write_all(processed.as_bytes());
                            }
                            Err(_) => break,
                        }
                    }
                    _ => break,
                }
            }
            let _ = stdout.flush();
            return Ok(0);
        }

        // Check for data from PTY
        match session.has_data(100) {
            Ok(true) => {
                match session.read(&mut output_buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let processed = processor.process_bytes(&output_buf[..n]);
                        stdout.write_all(processed.as_bytes())?;
                        stdout.flush()?;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok(false) => {
                // No data from PTY, check stdin
                // Use non-blocking read on stdin
                let mut readfds: libc::fd_set = unsafe { std::mem::zeroed() };
                unsafe { libc::FD_ZERO(&mut readfds); }
                unsafe { libc::FD_SET(0, &mut readfds); }

                let mut timeout = libc::timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                };

                let result = unsafe {
                    libc::select(
                        1,
                        &mut readfds,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut timeout,
                    )
                };

                if result > 0 && unsafe { libc::FD_ISSET(0, &readfds) } {
                    match stdin.read(&mut input_buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            session.write(&input_buf[..n])?;
                        }
                        Err(_) => break,
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
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
