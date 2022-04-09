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

/// Prints the message as a success.
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => ({
        $crate::log::success(&*format!($($arg)*));
    })
}

/// Prints the message as an error.
pub fn error(message: &str) {
    if _error(message).is_err() {
        println!("   ERROR {}", message);
    }
}

/// Prints the message as a success.
pub fn success(message: &str) {
    if _success(message).is_err() {
        println!(" SUCCESS {}", message);
    }
}

fn _error(message: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
    write!(stdout, "   ERROR ")?;
    stdout.reset()?;
    writeln!(stdout, "{}", message.replace('\n', "\n         "))
}

fn _success(message: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    write!(stdout, " SUCCESS ")?;
    stdout.reset()?;
    writeln!(stdout, "{}", message.replace('\n', "\n         "))
}

pub(crate) fn stdout() -> StandardStream {
    StandardStream::stdout(color_choice(Stream::Stdout))
}

fn color_choice(stream: Stream) -> ColorChoice {
    if atty::is(stream) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    }
}
