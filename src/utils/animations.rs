use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

const THINKING_FRAMES: [&str; 6] = [
    "⠋ Thinking...",
    "⠙ Thinking...",
    "⠹ Thinking...",
    "⠸ Thinking...",
    "⠼ Thinking...",
    "⠴ Thinking..."
];

pub async fn update_thinking_animation(frame: usize) {
    print!("\r\x1B[38;5;39m{}\x1B[0m", THINKING_FRAMES[frame % THINKING_FRAMES.len()]);
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(100)).await;
}