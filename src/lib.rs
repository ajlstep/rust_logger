use std::fs;
use std::io::{Error, Write};
use chrono::Local;

pub struct Logger {
    warn_log_file: String,
    info_log_file: String,
    err_log_file: String,
    panic_log_file: String,
    fatal_log_file: String,
    conection_log_file: String,
}

impl Logger {
    pub fn new() -> Logger {
        let now = Local::now().format("%Y-%m-%d_%H-%M-%S");

        let warn_log_file = format!("logs/warnings_{}.log", now);
        let info_log_file = format!("logs/informations_{}.log", now);
        let err_log_file = format!("logs/errors_{}.log", now);
        let panic_log_file = format!("logs/panics_{}.log", now);
        let fatal_log_file = format!("logs/fatals_{}.log", now);
        let conection_log_file = format!("logs/conection_{}.log", now);

        Logger {
            warn_log_file,
            info_log_file,
            err_log_file,
            panic_log_file,
            fatal_log_file,
            conection_log_file,
        }
    }

    fn write_log_to_file(&self, file: &str, log: &str) -> Result<(), Error> {
        if !fs::metadata("logs").is_ok() {
            fs::create_dir("logs")?;
        }

        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(file)?;

        f.write(log.as_bytes())?;
        // f.write_all(log.as_bytes())?;
        Ok(())
    }

    pub fn warn_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ WARN! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.warn_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }

    pub fn info_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ INFO! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.info_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }

    pub fn error_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ ERROR! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.err_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }

    pub fn panic_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ FATAL! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.panic_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }

    pub fn fatal_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ FATAL! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.fatal_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }

    pub fn connection_logger(&self, message: &str, path: &str, body: &str) {
        let now = Local::now();
        let log = format!("[ CONNECTION! ] Path: {} | Message: {} | Body: {}, {} ", path, message, body, now.format("%Y-%m-%d %H:%M:%S"));
        match self.write_log_to_file(&self.conection_log_file, &log) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }       
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_warn_logger() {
        let logger = Logger::new();
        logger.warn_logger("This is a warning message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.warn_log_file).exists());
    }

    #[test]
    fn test_info_logger() {
        let logger = Logger::new();
        logger.info_logger("This is an information message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.info_log_file).exists());
    }

    #[test]
    fn test_error_logger() {
        let logger = Logger::new();
        logger.error_logger("This is an error message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.err_log_file).exists());
    }

    #[test]
    fn test_panic_logger() {
        let logger = Logger::new();
        logger.panic_logger("This is a panic message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.panic_log_file).exists());
    }

    #[test]
    fn test_fatal_logger() {
        let logger = Logger::new();
        logger.fatal_logger("This is a fatal message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.fatal_log_file).exists());
    }

    #[test]
    fn test_conection_logger() {
        let logger = Logger::new();
        logger.connection_logger("This is a conection message", "path/to/file", "Some additional information");

        // Assert that the log file was created
        assert!(Path::new(&logger.conection_log_file).exists());
    }
}
