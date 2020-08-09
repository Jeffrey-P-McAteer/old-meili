
/**
 * The Global struct is responsible for
 * sharing mutable state among many different components
 * of meili.
 */

use std::sync::Mutex;

#[derive(Debug)]
pub struct Global {
  pub scan_ips_in_background: Mutex<bool>,
}

impl Default for Global {
  fn default() -> Self {
    Global {
      scan_ips_in_background: Mutex::new(false),
    }
  }
}

impl Global {
  pub fn set_scan_ips_in_background(&self, val: bool) {
    if let Ok(mut scan_ips_in_background) = self.scan_ips_in_background.lock() {
      *scan_ips_in_background = val;
    }
  }
  pub fn get_scan_ips_in_background(&self) -> bool {
    if let Ok(scan_ips_in_background) = self.scan_ips_in_background.lock() {
      return *scan_ips_in_background;
    }
    return false;
  }
}
