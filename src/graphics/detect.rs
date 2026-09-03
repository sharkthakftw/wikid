use super::protocol::GraphicsProtocol;
use std::env;
use std::sync::OnceLock;

static DETECTED_PROTOCOL: OnceLock<GraphicsProtocol> = OnceLock::new();

pub fn detect_terminal_protocol() -> GraphicsProtocol {
    *DETECTED_PROTOCOL.get_or_init(|| {
        let is_kitty = [
            "KITTY_WINDOW_ID",
            "GHOSTTY_RESOURCES_DIR",
            "WEZTERM_EXECUTABLE",
        ]
        .iter()
        .any(|k| env::var(k).is_ok())
            || ["TERM_PROGRAM", "TERM"].iter().any(|k| {
                env::var(k)
                    .map(|v| {
                        let l = v.to_lowercase();
                        l.contains("kitty") || l.contains("ghostty") || l.contains("wezterm")
                    })
                    .unwrap_or(false)
            });

        if is_kitty {
            GraphicsProtocol::Kitty
        } else {
            GraphicsProtocol::Halfblocks
        }
    })
}
