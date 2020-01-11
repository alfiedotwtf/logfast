//! LogFast - Insanely fast logging with a simple interface
//!
//! # Example
//!
//! ```ignore
//! use logfast::LogFast;
//! 
//! fn main() {
//!   let mut lf = LogFast::new("my.log", 100).unwrap();
//! 
//!   ...
//! 
//!   lf.log("Here's a test log line");
//!   lf.log("And here's another");
//! 
//!   ...
//! }
//! ```

use chrono::Local;
use die::die;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

/// Holds the LogFast housekeeping
pub struct LogFast {
    /// filename that the LogFast thread is writing to 
    pub filename: String,
    src: Option<SyncSender<String>>,
    barrier: Arc<Barrier>,
}

impl LogFast {
    /// Creates a new `LogFast` thread logger
    ///
    /// # Parameters
    ///
    /// `filename` is the path to the file we want to write logs to
    ///
    /// `buffer_size` is the size of the buffer to hold yet-to-be-flushed messages
    ///
    /// # Example
    ///
    /// ```ignore
    /// let lf = match LogFast::new("my.log", 100).unwrap();
    /// ```
    pub fn new(filename: &str, buffer_size: usize) -> Result<LogFast, String> {
        if buffer_size == 0 {
            return Err("Buffer size needs to be bigger than 0".to_string());
        }

        let mut log_file = match OpenOptions::new().create(true).append(true).open(filename) {
            Ok(log_file) => log_file,
            Err(err) => return Err(format!("Error opening log file '{}': {}", filename, err)),
        };

        let (src, dest) = sync_channel::<String>(buffer_size);
        let barrier = Arc::new(Barrier::new(2));

        {
            let barrier_moved = barrier.clone();
            let filename_moved = filename.to_string();

            thread::spawn(move || {
                while let Ok(msg) = dest.recv() {
                    writeln!(log_file, "{}", msg).unwrap_or_else(|err| {
                        die!("Error writing to log file '{}': {}", filename_moved, err)
                    })
                }

                barrier_moved.wait();
            })
        };

        Ok(LogFast{
            filename: filename.to_string(),
            src: Some(src),
            barrier: barrier,
        })
    }

    /// Send a new log message to the `LogFast` thread, to be flushed ASAP
    ///
    /// # Parameters
    ///
    /// `msg` is the log message you want to eventually be written
    ///
    /// # Example
    ///
    /// ```ignore
    /// lf.log("Here's a test log line");
    /// ```
    pub fn log(&mut self, msg: &str) {
        self.src
            .as_ref()
            .unwrap_or_else(|| {
                die!(
                    "Error talking to logging thread for log file '{}'",
                    self.filename
                )
            })
            .send(format!("{}: {}", Local::now().naive_local(), &msg))
            .unwrap_or_else(|err| {
                die!(
                    "Error talking to logging thread for log file '{}': {}",
                    self.filename,
                    err
                )
            })
    }
}

impl Drop for LogFast {
    fn drop(&mut self) {
        self.src = None;
        self.barrier.wait();
    }
}
