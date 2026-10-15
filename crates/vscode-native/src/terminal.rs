use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Terminal {
    pub output: Arc<Mutex<String>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    _pty_master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    _child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        #[cfg(target_os = "windows")]
        let default_shell = "powershell.exe";
        #[cfg(not(target_os = "windows"))]
        let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());

        let mut cmd = CommandBuilder::new(&default_shell);
        if let Ok(current_dir) = std::env::current_dir() {
            cmd.cwd(current_dir);
        }

        let child = pair.slave.spawn_command(cmd)?;
        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = Arc::clone(&output);

        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buf[..n]);
                let clean = strip_ansi_escapes::strip_str(&s);
                if let Ok(mut text) = output_clone.lock() {
                    text.push_str(&clean);
                    if text.len() > 50000 {
                        let drain_len = text.len() - 40000;
                        text.drain(..drain_len);
                    }
                }
            }
        });

        Ok(Self {
            output,
            writer: Arc::new(Mutex::new(writer)),
            _pty_master: Arc::new(Mutex::new(pair.master)),
            _child: child,
        })
    }

    #[allow(dead_code)]
    pub fn send_input(&self, input: &str) {
        if let Ok(mut w) = self.writer.lock() {
            let _ = w.write_all(input.as_bytes());
            let _ = w.flush();
        }
    }
}

mod strip_ansi_escapes {
    pub fn strip_str(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut in_escape = false;
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1B' {
                in_escape = true;
                if let Some(&'[') = chars.peek() {
                    chars.next();
                }
            } else if in_escape {
                if ch.is_ascii_alphabetic() {
                    in_escape = false;
                }
            } else {
                result.push(ch);
            }
        }
        result
    }
}
