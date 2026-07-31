//! Update checker from GitHub releases.

use crate::update::CheckUpdateResult;

pub fn is_newer_version(current: &str, latest: &str) -> bool {
  let current_parts: Vec<u32> = current
    .trim_start_matches('v')
    .split('.')
    .filter_map(|s| s.parse().ok())
    .collect();
  let latest_parts: Vec<u32> = latest
    .trim_start_matches('v')
    .split('.')
    .filter_map(|s| s.parse().ok())
    .collect();

  for (c, l) in current_parts.iter().zip(latest_parts.iter()) {
    if l > c {
      return true;
    }
    if l < c {
      return false;
    }
  }
  latest_parts.len() > current_parts.len()
}

pub async fn check_for_update(
  _repo: &str,
  _current_version: &str,
) -> Result<CheckUpdateResult, String> {
  // Simplified - actual implementation would call GitHub API
  Ok(CheckUpdateResult {
    has_update: false,
    update_info: None,
    error: Some("Update checking not implemented".to_string()),
  })
}

pub fn find_platform_asset<'a>(_assets: &'a [crate::update::GitHubAsset], _platform: &str) -> Option<&'a crate::update::GitHubAsset> {
  None
}
