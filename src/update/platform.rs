//! Platform detection for updates.

#[derive(Debug, Clone)]
pub enum Platform {
  Windows,
  MacOS,
  Linux,
  Unknown,
}

impl Platform {
  pub fn current() -> Self {
    #[cfg(target_os = "windows")]
    return Platform::Windows;
    #[cfg(target_os = "macos")]
    return Platform::MacOS;
    #[cfg(target_os = "linux")]
    return Platform::Linux;
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Platform::Unknown;
  }

  pub fn as_str(&self) -> &str {
    match self {
      Platform::Windows => "windows",
      Platform::MacOS => "macos",
      Platform::Linux => "linux",
      Platform::Unknown => "unknown",
    }
  }
}
