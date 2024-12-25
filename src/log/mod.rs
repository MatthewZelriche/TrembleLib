use std::path::Path;

use chrono::Local;
use colored::Colorize;

struct Logger;

pub fn initialize_logger() {
    let _ = log::set_boxed_logger(Box::new(Logger {}))
        .map(|()| log::set_max_level(log::LevelFilter::Trace)); // TODO: Allow proper setting of log level
}

impl Logger {
    fn color_to_term(&self, level: log::Level, format_msg: &str) {
        match level {
            log::Level::Error => println!("{}", format_msg.bright_white().on_red()),
            log::Level::Warn => println!("{}", format_msg.bright_yellow()),
            log::Level::Info => println!("{}", format_msg.white()),
            _ => println!("{}", format_msg.cyan()),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true // TODO
    }

    fn log(&self, record: &log::Record) {
        let file = Path::new(record.file().unwrap_or_default())
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let line = record.line().unwrap_or_default();
        let level = record.level();
        let time = Local::now().format("%H:%M:%S%.6f");

        let formatted = format!(
            "{:.4} | {} | {:>20.20}:{:0>4} | {}",
            level,
            time,
            file,
            line,
            record.args()
        );
        self.color_to_term(level, &formatted);

        // TODO: File logging

        self.flush(); // Always flush, over FFI our string pointers are pinned when it comes to C# runtime
    }

    fn flush(&self) {}
}
