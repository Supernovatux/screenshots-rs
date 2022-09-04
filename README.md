# This fork returns `image::DynamicImage` instead of the custom image which the library uses. Works only on Linux
# Bechmark: vs upstream on x11
|   | Original  | This fork  |
|---|---|---|
| Without saving to disk |   | 20ms  |
| With saving to disk  |   | 200ms  |
# screenshots

A screenshots library for Linux(X11、wayland).

## example

```rust
use screenshots::Screen;
use std::{time::Instant};

fn main() {
  let start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    let mut image = screen.capture().unwrap();
    println!("{}",image.height());
    //image.save(format!("{}.png", screen.display_info.id)).unwrap();

    image = screen.capture_area(300, 300, 300, 300).unwrap();
    println!("{}",image.height());
    //image.save(format!("{}-2.png", screen.display_info.id)).unwrap();
  }

  let screen = Screen::from_point(100, 100).unwrap();
  println!("capturer {:?}", screen);

  let image = screen.capture_area(300, 300, 300, 300).unwrap();
  println!("{}",image.height());
  //image.save("capture_display_with_point.png").unwrap();

  println!("运行耗时: {:?}", start.elapsed());
}



```

## API

### `Screen`: Screen capturer

- `Screen::new(display_info)`: Get screen from [display info](https://docs.rs/display-info/latest/display_info/struct.DisplayInfo.html), return `Option<Screen>`.
- `Screen::all()`: Get all screen, return `Vec<Screen>`.
- `Screen::from_point(x, y)`: Get screen from point, return `Option<Screen>`.
- `screen.capture()`: capture screen screenshot [image](https://docs.rs/screenshots/latest/screenshots/struct.Image.html), return `Option<DynamicImage>`.
- `screen.capture_area(x, y, width, height)`: Capture the current screen the designated area, return `Option<DynamicImage>`.


## Linux requirements

On Linux, you need to install `libxcb`、`libxrandr`、`dbus`

Debian/Ubuntu:

```sh
apt-get install libxcb1 libxrandr2 libdbus-1-3
```

Alpine:

```sh
apk add libxcb libxrandr dbus
```