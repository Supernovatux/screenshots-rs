use crate::DisplayInfo;
use dbus::{self, blocking::Connection};
use image::DynamicImage;
use std::{
  env::temp_dir,
  fs,
  time::{Duration, SystemTime, UNIX_EPOCH},
};

fn screenshot(x: i32, y: i32, width: i32, height: i32) -> Result<String, dbus::Error> {
  let conn = Connection::new_session()?;

  let proxy = conn.with_proxy(
    "org.gnome.Shell.Screenshot",
    "/org/gnome/Shell/Screenshot",
    Duration::from_secs(10),
  );

  let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_micros().to_string(),
    Err(_) => return Err(dbus::Error::new_failed("Get system timestamp failed")),
  };

  let dirname = temp_dir().join("screenshot");

  fs::create_dir_all(&dirname)
    .map_err(|_| dbus::Error::new_failed(format!("Create dir {:?} failed", dirname).as_str()))?;

  let mut path = dirname.join(timestamp);
  path.set_extension("png");

  let filename = path.to_string_lossy().to_string();

  let _: () = proxy.method_call(
    "org.gnome.Shell.Screenshot",
    "ScreenshotArea",
    (x, y, width, height, false, &filename),
  )?;

  Ok(filename)
}

pub fn wayland_capture_screen(display_info: &DisplayInfo) -> Option<DynamicImage> {
  let x = ((display_info.x as f32) * display_info.scale_factor) as i32;
  let y = ((display_info.y as f32) * display_info.scale_factor) as i32;
  let width = (display_info.width as f32) * display_info.scale_factor;
  let height = (display_info.height as f32) * display_info.scale_factor;

  let filename = screenshot(x, y, width as i32, height as i32).ok()?;
  image::open(filename).ok()
}
pub fn wayland_capture_screen_raw(display_info: &DisplayInfo) -> Option<Vec<u8>> {
  let x = ((display_info.x as f32) * display_info.scale_factor) as i32;
  let y = ((display_info.y as f32) * display_info.scale_factor) as i32;
  let width = (display_info.width as f32) * display_info.scale_factor;
  let height = (display_info.height as f32) * display_info.scale_factor;

  let filename = screenshot(x, y, width as i32, height as i32).ok()?;
  std::fs::read(filename).ok()
}

pub fn wayland_capture_screen_area(
  display_info: &DisplayInfo,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
) -> Option<DynamicImage> {
  let area_x = (((x + display_info.x) as f32) * display_info.scale_factor) as i32;
  let area_y = (((y + display_info.y) as f32) * display_info.scale_factor) as i32;
  let area_width = (width as f32) * display_info.scale_factor;
  let area_height = (height as f32) * display_info.scale_factor;

  let filename = screenshot(area_x, area_y, area_width as i32, area_height as i32).ok()?;
  image::open(filename).ok()
}
