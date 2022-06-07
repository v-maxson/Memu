//! A cross-platform logging utility.
#![allow(unused)]

use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub enum LogType {
    Error,
    Warning,
    Info,
    Debug
}

impl std::fmt::Debug for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
        }
    }
}

impl LogType {
    fn get_color(&self) -> Color {
        match self {
            LogType::Error => Color::Red,
            LogType::Warning => Color::Yellow,
            LogType::Info => Color::Green,
            LogType::Debug => Color::Blue
        }
    }
}

#[inline(always)]
pub fn colored_log(message: String, t: LogType) {
    let buffer_writer = if let LogType::Error = t {
        BufferWriter::stderr(ColorChoice::Always)
    } else {
        BufferWriter::stdout(ColorChoice::Always)
    };

    let mut buffer = buffer_writer.buffer();
    _ = buffer.set_color(ColorSpec::new().set_bold(true).set_fg(Some(t.get_color())));
    _ = write!(&mut buffer, "{:?}: ", t);
    _ = buffer.reset();

    _ = writeln!(&mut buffer, "{}", message);
    _ = buffer_writer.print(&buffer);
}

#[macro_export]
macro_rules! error {
    ($x:expr) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x), LogType::Error);
        }
    };
    ($x:expr, $($arg:tt)*) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x, $($arg)*), LogType::Error);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($x:expr) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x), LogType::Warning);
        }
    };
    ($x:expr, $($arg:tt)*) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x, $($arg)*), LogType::Warning);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($x:expr) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x), LogType::Info);
        }
    };
    ($x:expr, $($arg:tt)*) => {
        {
            use $crate::logger::{LogType, colored_log};
            colored_log(format!($x, $($arg)*), LogType::Info);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($x:expr) => {
        {
            use $crate::constants::load_debug;

            if load_debug() {
                use $crate::logger::{LogType, colored_log};
                colored_log(format!($x), LogType::Debug);
            }
        }
    };
    ($x:expr, $($arg:tt)*) => {
        {
            use $crate::constants::load_debug;

            if load_debug() {
                use $crate::logger::{LogType, colored_log};
                colored_log(format!($x, $($arg)*), LogType::Debug);
            }
        }
    };
}