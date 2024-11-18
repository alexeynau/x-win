#![deny(unused_imports)]

mod common_api;
mod gnome_shell;
mod wayland_api;
mod wayland_eval_api;
mod wayland_extension_api;
mod x11_api;

use common_api::is_wayland_desktop;
use wayland_api::WaylandApi;
use x11_api::X11Api;

use crate::common::{
  api::Api,
  x_win_struct::{icon_info::IconInfo, window_info::WindowInfo},
};

pub trait APIGnome {
  fn install_extension() -> bool;
  fn uninstall_extension() -> bool;
  fn enable_extension() -> bool;
  fn disable_extension() -> bool;
}

pub struct LinuxAPI {}

/**
 * Impl. for windows system
 */
impl Api for LinuxAPI {
  fn get_active_window(&self) -> Result<WindowInfo, std::io::Error> {
    if is_wayland_desktop() {
      (WaylandApi {}).get_active_window()
    } else {
      (X11Api {}).get_active_window()
    }
  }

  fn get_open_windows(&self) -> Result<Vec<WindowInfo>, std::io::Error> {
    if is_wayland_desktop() {
      (WaylandApi {}).get_open_windows()
    } else {
      (X11Api {}).get_open_windows()
    }
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> IconInfo {
    if is_wayland_desktop() {
      (WaylandApi {}).get_app_icon(window_info)
    } else {
      (X11Api {}).get_app_icon(window_info)
    }
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> String {
    if is_wayland_desktop() {
      (WaylandApi {}).get_browser_url(window_info)
    } else {
      (X11Api {}).get_browser_url(window_info)
    }
  }
}

impl APIGnome for LinuxAPI {
  fn install_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::install_extension()
    } else {
      false
    }
  }

  fn uninstall_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::uninstall_extension()
    } else {
      false
    }
  }

  fn enable_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::enable_extension()
    } else {
      false
    }
  }

  fn disable_extension() -> bool {
    if is_wayland_desktop() {
      WaylandApi::disable_extension()
    } else {
      false
    }
  }
}
