use crate::emoji::Emoji;
use atty::Stream;
use std::io::{Result, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Prints the message as an error.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::log::error(&*format!($($arg)*));
    })
}

/// Prints the message as a warn.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::log::warn(&*format!($($arg)*));
    })
}

/// Prints the message as a notice.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ({
        $crate::log::trace(&*format!($($arg)*));
    })
}

/// Prints the message as an error.
pub fn error(message: &str) {
    if _error(message).is_err() {
        println!("* {}", message);
    }
}

/// Prints the message as a warn.
pub fn warn(message: &str) {
    if _warn(message).is_err() {
        println!("$ {}", message);
    }
}

/// Prints the message as a notice.
pub fn trace(message: &str) {
    if _trace(message).is_err() {
        println!("> {}", message);
    }
}

fn _error(message: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln_with_emoji(&mut stdout, "❌", "*", message)
}

fn _warn(message: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln_with_emoji(&mut stdout, "⚡", "$", message)
}

fn _trace(message: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln_with_emoji(&mut stdout, "🏁", "#", message)
}

fn writeln_with_emoji(
    stream: &mut StandardStream,
    emoji: &str,
    fallback: &str,
    message: &str,
) -> Result<()> {
    write!(
        stream,
        "{} {}",
        Emoji::new(emoji, fallback),
        message.replace('\n', Emoji::new("\n   ", "\n  ").string())
    )?;
    stream.reset()?;
    writeln!(stream)
}

fn stdout() -> StandardStream {
    StandardStream::stdout(color_choice(Stream::Stdout))
}

fn color_choice(stream: Stream) -> ColorChoice {
    if atty::is(stream) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    }
}
