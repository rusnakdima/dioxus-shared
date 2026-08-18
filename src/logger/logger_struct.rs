//! Global logger instance.

use std::sync::{Arc, Mutex, OnceLock, RwLock};

use super::file_writer::FileLogger;
use super::log_entry::LogEntry;
use super::log_level::LogLevel;

static LOGGER: OnceLock<Logger> = OnceLock::new();
static LOG_LEVEL: OnceLock<RwLock<LogLevel>> = OnceLock::new();

pub struct Logger {
    entries: Mutex<Vec<LogEntry>>,
    max_entries: usize,
    file_logger: Arc<RwLock<Option<FileLogger>>>,
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        let entries_guard = match self.entries.lock() {
            Ok(g) => g,
            Err(_) => return Self::new(self.max_entries),
        };
        Self {
            entries: Mutex::new(entries_guard.clone()),
            max_entries: self.max_entries,
            file_logger: self.file_logger.clone(),
        }
    }
}

impl Logger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
            max_entries,
            file_logger: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_file_logger(self, path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        {
            let mut guard = self
                .file_logger
                .write()
                .map_err(|_| std::io::Error::other("File logger lock poisoned"))?;
            *guard = Some(FileLogger::new(path)?);
        }
        Ok(self)
    }

    pub fn global() -> &'static Logger {
        LOGGER.get_or_init(|| Logger::new(1000))
    }

    pub fn global_level() -> &'static RwLock<LogLevel> {
        LOG_LEVEL.get_or_init(|| RwLock::new(LogLevel::Info))
    }

    pub fn set_level(level: LogLevel) {
        if let Ok(mut guard) = Self::global_level().write() {
            *guard = level;
        }
    }

    pub fn get_level() -> Result<LogLevel, String> {
        let guard = Self::global_level()
            .read()
            .map_err(|_| "Log level lock poisoned".to_string())?;
        Ok(guard.clone())
    }

    pub fn log(&self, level: LogLevel, message: &str, source: Option<&str>) {
        let min_level = match Self::get_level() {
            Ok(l) => l,
            Err(_) => return,
        };
        if !level.is_enabled(min_level) {
            return;
        }

        let entry = LogEntry {
            level,
            message: message.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            source: source.map(String::from),
            metadata: None,
        };

        let mut entries = match self.entries.lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        entries.push(entry.clone());

        if entries.len() > self.max_entries {
            entries.remove(0);
        }

        if let Ok(mut fl) = self.file_logger.write() {
            if let Some(ref mut file_logger) = *fl {
                let _ = file_logger.write(&entry);
            }
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message, None);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message, None);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message, None);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message, None);
    }

    pub fn get_entries(&self) -> Result<Vec<LogEntry>, String> {
        let guard = self
            .entries
            .lock()
            .map_err(|_| "Logger entries lock poisoned".to_string())?;
        Ok(guard.clone())
    }

    pub fn clear(&self) -> Result<(), String> {
        let mut guard = self
            .entries
            .lock()
            .map_err(|_| "Logger entries lock poisoned".to_string())?;
        guard.clear();
        Ok(())
    }
}
