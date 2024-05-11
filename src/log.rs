//! Macros for multi-level formatted logging used by
//! [ThrillerFlow](https:://github.com/TiledTensor/ThrillerFlow).
//!
//!
//! The log macros, in descending order of level, are: [`error!`], [`warn!`],
//! [`info!`], [`debug!`], and [`trace!`].

use std::fmt::{self, Write};
use std::str::FromStr;

use log::{Level, LevelFilter, Log};

pub use log::{debug, error, info, trace, warn};

/// Prints to the console, with a newline.
macro_rules! with_color {
    ($color_code:expr, $($arg:tt)*) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
    }};
}

#[repr(u8)]
#[allow(dead_code)]
enum ColorCode {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

struct Logger;

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        print!("{}", s);
        Ok(())
    }
}

/// Prints the formatted string to the console.
pub fn print_fmt(args: fmt::Arguments) -> fmt::Result {
    Logger.write_fmt(args)
}

#[doc(hidden)]
pub fn __print_impl(args: fmt::Arguments) {
    print_fmt(args).unwrap();
}

impl Log for Logger {
    #[inline]
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();
        let line = record.line().unwrap_or(0);
        let path = record.target();
        let args_color = match level {
            Level::Error => ColorCode::Red,
            Level::Warn => ColorCode::Yellow,
            Level::Info => ColorCode::Green,
            Level::Debug => ColorCode::Cyan,
            Level::Trace => ColorCode::BrightBlack,
        };

        __print_impl(with_color!(
            ColorCode::White,
            "[{time} {path}:{line}] {args}\n",
            time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            path = path,
            line = line,
            args = with_color!(args_color, "{}", record.args()),
        ));
    }

    fn flush(&self) {}
}

/// Initializes the logger.
///
/// This function should be called before any log macros are used, otherwise
/// nothing will be printed.
pub fn init_logger() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(LevelFilter::Warn);
}

/// Set the maximum log level.
///
/// Unlike the features such as `log-level-error`, setting the logging level in
/// this way incurs runtime overhead. In addition, this function is no effect
/// when those features are enabled.
///
/// `level` should be one of `off`, `error`, `warn`, `info`, `debug`, `trace`.
pub fn set_max_level(level: &str) {
    let lf = LevelFilter::from_str(level)
        .ok()
        .unwrap_or(LevelFilter::Off);
    log::set_max_level(lf);
}
