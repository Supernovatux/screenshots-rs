use crate::DisplayInfo;
use image::DynamicImage;
use xcb::x::{Drawable, GetImage, ImageFormat};

fn capture_raw(x: i32, y: i32, width: u32, height: u32) -> Option<Vec<u8>> {
  let (conn, index) = xcb::Connection::connect(None).ok()?;

  let setup = conn.get_setup();
  let screen = setup.roots().nth(index as usize)?;

  let get_image_cookie = conn.send_request(&GetImage {
    format: ImageFormat::ZPixmap,
    drawable: Drawable::Window(screen.root()),
    x: x as i16,
    y: y as i16,
    width: width as u16,
    height: height as u16,
    plane_mask: u32::MAX,
  });

  let get_image_reply = conn.wait_for_reply(get_image_cookie).ok()?;
  Some(Vec::from(get_image_reply.data()))
}
fn capture(x: i32, y: i32, width: u32, height: u32) -> Option<DynamicImage> {
  let (conn, index) = xcb::Connection::connect(None).ok()?;

  let setup = conn.get_setup();
  let screen = setup.roots().nth(index as usize)?;

  let get_image_cookie = conn.send_request(&GetImage {
    format: ImageFormat::ZPixmap,
    drawable: Drawable::Window(screen.root()),
    x: x as i16,
    y: y as i16,
    width: width as u16,
    height: height as u16,
    plane_mask: u32::MAX,
  });

  let get_image_reply = conn.wait_for_reply(get_image_cookie).ok()?;
  let bytes = Vec::from(get_image_reply.data());
  let rgb_image = image::RgbaImage::from_raw(width, height, bytes).unwrap();
  Some(image::DynamicImage::ImageRgba8(rgb_image))
}

pub fn xorg_capture_screen(display_info: &DisplayInfo) -> Option<DynamicImage> {
  let x = ((display_info.x as f32) * display_info.scale_factor) as i32;
  let y = ((display_info.y as f32) * display_info.scale_factor) as i32;
  let width = ((display_info.width as f32) * display_info.scale_factor) as u32;
  let height = ((display_info.height as f32) * display_info.scale_factor) as u32;

  capture(x, y, width, height)
}
pub fn xorg_capture_screen_raw(display_info: &DisplayInfo) -> Option<Vec<u8>> {
  let x = ((display_info.x as f32) * display_info.scale_factor) as i32;
  let y = ((display_info.y as f32) * display_info.scale_factor) as i32;
  let width = ((display_info.width as f32) * display_info.scale_factor) as u32;
  let height = ((display_info.height as f32) * display_info.scale_factor) as u32;

  capture_raw(x, y, width, height)
}
pub fn xorg_capture_screen_area(
  display_info: &DisplayInfo,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
) -> Option<DynamicImage> {
  let area_x = (((x + display_info.x) as f32) * display_info.scale_factor) as i32;
  let area_y = (((y + display_info.y) as f32) * display_info.scale_factor) as i32;
  let area_width = ((width as f32) * display_info.scale_factor) as u32;
  let area_height = ((height as f32) * display_info.scale_factor) as u32;

  capture(area_x, area_y, area_width, area_height)
}
