//! File writer for persistent logging.

use super::log_entry::LogEntry;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct FileLogger {
  writer: BufWriter<File>,
}

impl FileLogger {
  pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(path)?;
    Ok(Self {
      writer: BufWriter::new(file),
    })
  }

  pub fn write(&mut self, entry: &LogEntry) -> Result<(), std::io::Error> {
    writeln!(
      self.writer,
      "[{}] {} - {}",
      entry.timestamp,
      entry.level,
      entry.message
    )?;
    self.writer.flush()?;
    Ok(())
  }
}
