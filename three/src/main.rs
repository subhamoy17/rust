use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Write, Error};
use std::path::Path;

trait Logger {
    fn log(&mut self, message: &str);
}

// Console logger for fallback/debug
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("[Console] {}", message);
    }
}

// File logger with panic on critical error
struct FileLogger {
    file: File,
}

impl FileLogger {
    fn new(path: &str) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(path))?;

        Ok(Self { file })
    }
}

impl Logger for FileLogger {
    fn log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "[File] {}", message) {
            // Critical failure: cannot log!
            panic!("Critical logging failure: {}", e);
        }
    }
}

// Simulates the main application
fn main() {
    println!("--- Logger Start ---");

    // Create logger
    let mut logger: Box<dyn Logger> = match FileLogger::new("app.log") {
        Ok(file_logger) => Box::new(file_logger),
        Err(e) => {
            println!("File logger failed: {}. Falling back to console.", e);
            Box::new(ConsoleLogger)
        }
    };

    // Logging messages
    logger.log("System booted.");
    logger.log("Running transaction...");
    logger.log("Logging finished.");

    println!("--- Logger End ---");
}
