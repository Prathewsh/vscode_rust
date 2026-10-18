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
            rows: 32,
            cols: 140,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        #[cfg(target_os = "windows")]
        let default_shell = "powershell.exe";
        #[cfg(not(target_os = "windows"))]
        let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());

        let mut cmd = CommandBuilder::new(&default_shell);
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        if let Ok(current_dir) = std::env::current_dir() {
            cmd.cwd(current_dir);
        }

        let child = pair.slave.spawn_command(cmd)?;
        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = Arc::clone(&output);

        thread::spawn(move || {
            let mut buf = [0u8; 2048];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buf[..n]);
                let clean = strip_ansi_escapes::clean_terminal_output(&s);
                if let Ok(mut text) = output_clone.lock() {
                    text.push_str(&clean);
                    if text.len() > 60000 {
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

    pub fn send_input(&self, input: &str) {
        if let Ok(mut w) = self.writer.lock() {
            let _ = w.write_all(input.as_bytes());
            let _ = w.flush();
        }
    }

    pub fn clear(&self) {
        if let Ok(mut text) = self.output.lock() {
            text.clear();
        }
    }
}

#[allow(dead_code)]
pub struct TerminalSession {
    pub id: usize,
    pub name: String,
    pub terminal: Terminal,
}

pub struct TerminalManager {
    pub sessions: Vec<TerminalSession>,
    pub active_idx: usize,
    next_id: usize,
}

impl TerminalManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            sessions: Vec::new(),
            active_idx: 0,
            next_id: 1,
        };
        mgr.new_session(None);
        mgr
    }

    pub fn new_session(&mut self, name: Option<&str>) {
        if let Ok(term) = Terminal::new() {
            let session_name = name.unwrap_or("zsh").to_string();
            let session = TerminalSession {
                id: self.next_id,
                name: session_name,
                terminal: term,
            };
            self.next_id += 1;
            self.sessions.push(session);
            self.active_idx = self.sessions.len() - 1;
        }
    }

    pub fn close_session(&mut self, idx: usize) {
        if idx < self.sessions.len() {
            self.sessions.remove(idx);
            if self.active_idx >= self.sessions.len() {
                self.active_idx = self.sessions.len().saturating_sub(1);
            }
        }
    }

    pub fn active_session(&self) -> Option<&TerminalSession> {
        self.sessions.get(self.active_idx)
    }

    #[allow(dead_code)]
    pub fn active_session_mut(&mut self) -> Option<&mut TerminalSession> {
        self.sessions.get_mut(self.active_idx)
    }
}

mod strip_ansi_escapes {
    pub fn clean_terminal_output(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];
            if ch == '\x1B' {
                i += 1;
                if i < len && chars[i] == '[' {
                    i += 1;
                    // Skip parameters until command character
                    while i < len && (chars[i].is_ascii_digit() || chars[i] == ';' || chars[i] == '?' || chars[i] == ' ') {
                        i += 1;
                    }
                    if i < len {
                        i += 1; // skip final command letter (e.g. 'm', 'H', 'J', 'K')
                    }
                } else if i < len && chars[i] == ']' {
                    // OSC sequence: skip until BEL (\x07) or ST (\x1b\)
                    i += 1;
                    while i < len && chars[i] != '\x07' && !(chars[i] == '\x1B' && i + 1 < len && chars[i + 1] == '\\') {
                        i += 1;
                    }
                    if i < len && chars[i] == '\x07' {
                        i += 1;
                    } else if i + 1 < len && chars[i] == '\x1B' && chars[i + 1] == '\\' {
                        i += 2;
                    }
                } else if i < len && (chars[i] == '(' || chars[i] == ')') {
                    i += 2; // character set selector
                }
            } else if ch == '\r' {
                if i + 1 < len && chars[i + 1] == '\n' {
                    result.push('\n');
                    i += 2;
                } else {
                    result.push('\n');
                    i += 1;
                }
            } else if ch != '\0' && ch != '\x07' {
                result.push(ch);
                i += 1;
            } else {
                i += 1;
            }
        }

        result
    }
}

