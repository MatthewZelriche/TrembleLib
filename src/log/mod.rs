use ansi_term::Color;
use chrono::Local;
use rotating_file::RotatingFile;
use std::io::Write;
use std::{path::Path, sync::Mutex};

mod rotating_file;

struct Logger {
    log_file: Option<Mutex<RotatingFile>>,
}

pub fn initialize_logger() {
    let logger = Box::new(Logger::new("engine.log", 5));
    let has_log_file = logger.has_log_file();
    // TODO: Allow proper setting of log level
    let _ = log::set_boxed_logger(logger).map(|()| log::set_max_level(log::LevelFilter::Trace));

    if !has_log_file {
        log::warn!("An error occured when setting up rotating log files. Logging to file may not function correctly.");
    }
}

impl Logger {
    pub fn new(filename: &str, max_rotates: u32) -> Self {
        let file = RotatingFile::new(filename, max_rotates)
            .ok()
            .map(|x| Mutex::new(x));
        let _ = ansi_term::enable_ansi_support();
        Logger { log_file: file }
    }

    pub fn has_log_file(&self) -> bool {
        self.log_file.is_some()
    }

    fn color_to_term(&self, level: log::Level, format_msg: &str) {
        match level {
            log::Level::Error => println!("{}", Color::White.on(Color::Red).paint(format_msg)),
            log::Level::Warn => println!("{}", Color::Yellow.paint(format_msg)),
            log::Level::Info => println!("{}", Color::White.paint(format_msg)),
            _ => println!("{}", Color::Cyan.paint(format_msg)),
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

        // Attempt to write to log file, silently fail if we cannot.
        self.log_file.as_ref().inspect(|x| {
            let _ = writeln!(x.lock().unwrap(), "{}", formatted);
        });
    }

    fn flush(&self) {
        self.log_file.as_ref().inspect(|x| {
            let _ = x.lock().unwrap().flush();
        });
    }
}
