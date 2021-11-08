//! lawg is a Rust library designed to log and create log files with ease.
//! # Example
//! ```rust
//! use lawg::Logger;
//!
//! fn main() {
//!     let logger = Logger::new(String::from("General Logger"), Some(String::from("../logs/general/logs.txt")), true);
//!
//!     logger.log("Started"); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: Started
//!     logger.log_to_file("Started again");
//!
//!     let mut x = 1 + 1;
//!
//!     if x == 2 {
//!         logger.log_and_log_to_file(String::from("It is two")); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: It is two
//!     } else {
//!         logger.error_and_stop("1 + 1 is not two"); // ERROR: My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: 1 + 1 is not two
//!     }
//! }
//! ```

use std::fs;

use chrono::Local;
use chrono::Utc;

/// The `Logger` struct, used for logging.
#[derive(Debug)]
pub struct Logger {
    pub logger_name: String,
    pub file_log: Option<String>,
    pub use_utc: bool,
}

impl Logger {
    /// Creates a new `Logger` struct.
    /// If `file_log` is provided, it will check if the file exists.
    /// If it does, it will do a read and write test on it, otherwise it will create a new file `file_log`.
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// let another_logger = Logger::new("My Another Logger", None, false);
    /// ```
    pub fn new(logger_name: String, file_log: Option<String>, use_utc: bool) -> Self {
        if let Some(file) = file_log.clone() {
            let mut file_log_content = String::new();

            if std::path::Path::new(&file.clone()).exists() {
                file_log_content = fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Could not read log file `{}`", file));
            }

            fs::write(file.clone(), file_log_content.as_bytes())
                .unwrap_or_else(|_| panic!("Could not create log file `{}`", file));
        }

        Logger {
            logger_name,
            file_log,
            use_utc,
        }
    }

    /// Logs to the console.
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.log("This is a log"); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: This is a log
    /// ```
    pub fn log<T: std::fmt::Display>(&self, msg: T) {
        let to_log = format!(
            "{} - [{:?}]: {}",
            self.logger_name,
            {
                if self.use_utc {
                    Utc::now().to_string()
                } else {
                    Local::now().to_string()
                }
            },
            msg
        );

        println!("{}", to_log);
    }

    /// Logs to file `Logger.file_log` (and not shown on the console).
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.log_to_file("This is log is written on the file and not shown on the console.");
    /// ```
    pub fn log_to_file<T: std::fmt::Display>(&self, msg: T) {
        if let Some(file_log) = &self.file_log {
            let file_log_content = fs::read_to_string(file_log.clone())
                .unwrap_or_else(|_| panic!("Could not read log file `{}`", file_log.clone()));

            fs::write(
                &file_log.clone(),
                (file_log_content
                    + "\n"
                    + &format!(
                        "{} - [{:?}]: {}",
                        self.logger_name,
                        {
                            if self.use_utc {
                                Utc::now().to_string()
                            } else {
                                Local::now().to_string()
                            }
                        },
                        msg
                    ))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Could not create log file `{}`", file_log.clone()));
        } else {
            panic!("Log file not provided.");
        }
    }

    /// Logs to the console and file `Logger.file_log`.
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.log_and_log_to_file("This log will appear on the console and also be written to the file"); // My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: This log will appear on the console and also be written to the file
    /// ```
    pub fn log_and_log_to_file(&self, msg: String) {
        self.log(msg.clone());
        self.log_to_file(msg);
    }

    /// Logs an error to the console.
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.error("Something went wrong! Try again later"); // ERROR: My Logger - ["yyyy-mm-dd hh:mm:ss UTC"]: Something went wrong! Try again later
    /// ```
    pub fn error<T: std::fmt::Display>(&self, msg: T) {
        let to_log = format!(
            "ERROR: {} - [{:?}]: {}",
            self.logger_name,
            {
                if self.use_utc {
                    Utc::now().to_string()
                } else {
                    Local::now().to_string()
                }
            },
            msg
        );

        println!("{}", to_log);
    }

    /// Logs an error to file `Logger.file_log` (and not shown on the console).
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.error_to_file("Something went wrong! Try again later");
    /// ```
    pub fn error_to_file<T: std::fmt::Display>(&self, msg: T) {
        if let Some(file_log) = &self.file_log {
            let file_log_content = fs::read_to_string(file_log.clone())
                .unwrap_or_else(|_| panic!("Could not read log file `{}`", file_log.clone()));

            fs::write(
                &file_log.clone(),
                (file_log_content
                    + "\n"
                    + &format!(
                        "ERROR: {} - [{:?}]: {}",
                        self.logger_name,
                        {
                            if self.use_utc {
                                Utc::now().to_string()
                            } else {
                                Local::now().to_string()
                            }
                        },
                        msg
                    ))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Could not create log file `{}`", file_log.clone()));
        } else {
            panic!("Log file not provided.");
        }
    }

    /// Logs an error to the console and file `Logger.file_log`.
    /// # Example
    /// ```rust
    /// use lawg::Logger;
    ///
    /// let my_logger = Logger::new("My Logger", Some("../logs/log_file.txt"), true);
    /// my_logger.error_to_file("Something went wrong! Try again later");
    /// ```
    pub fn error_and_error_to_file(&self, msg: String) {
        self.error(msg.clone());
        self.error_to_file(msg);
    }

    /// Logs an error to the console and stops the program.
    pub fn error_and_stop<T: std::fmt::Display>(&self, msg: T) {
        let to_log = format!(
            "ERROR: {} - [{:?}]: {}",
            self.logger_name,
            {
                if self.use_utc {
                    Utc::now().to_string()
                } else {
                    Local::now().to_string()
                }
            },
            msg
        );

        println!("{}", to_log);

        std::process::exit(1);
    }

    /// Logs an error to file `Logger.file_log` and stops the program.
    pub fn error_and_stop_to_file<T: std::fmt::Display>(&self, msg: T) {
        if let Some(file_log) = &self.file_log {
            let file_log_content = fs::read_to_string(file_log.clone())
                .unwrap_or_else(|_| panic!("Could not read log file `{}`", file_log.clone()));

            fs::write(
                &file_log.clone(),
                (file_log_content
                    + "\n"
                    + &format!(
                        "ERROR: {} - [{:?}]: {}",
                        self.logger_name,
                        {
                            if self.use_utc {
                                Utc::now().to_string()
                            } else {
                                Local::now().to_string()
                            }
                        },
                        msg
                    ))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Could not create log file `{}`", file_log.clone()));

            std::process::exit(1);
        } else {
            panic!("Log file not provided.");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::Logger;

        let my_logger = Logger::new("My Logger".to_string(), Some("log.txt".to_string()), true);

        my_logger.log("Hello world");
        my_logger.log_to_file("Hello world 123");
        my_logger.log_and_to_file("He ate my cereals");
    }
}
