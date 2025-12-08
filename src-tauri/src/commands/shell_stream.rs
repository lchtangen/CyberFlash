use tauri::{command, AppHandle, Emitter};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

#[command]
pub fn start_shell_stream(app: AppHandle, command: String, args: Vec<String>) {
    thread::spawn(move || {
        let mut child = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start shell command");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let _ = app.emit("shell-stdout", l);
                }
            }
        }
        
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let _ = app.emit("shell-stderr", l);
                }
            }
        }
    });
}
