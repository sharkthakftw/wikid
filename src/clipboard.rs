use std::io::Write;
use std::process::{Command, Stdio};

fn try_pipe_to_command(cmd: &str, args: &[&str], text: &str) -> bool {
    let mut child = match Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }

    child.wait().is_ok_and(|s| s.success())
}

pub fn copy_to_clipboard(text: &str) -> bool {
    const CANDIDATES: &[(&str, &[&str])] = &[
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
        ("pbcopy", &[]),
        ("clip.exe", &[]),
    ];

    CANDIDATES
        .iter()
        .any(|&(cmd, args)| try_pipe_to_command(cmd, args, text))
}
