use screenshots::Screen;
use std::{time::Instant};

fn main() {
  let start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    let mut image = screen.capture().unwrap();
    println!("{}",image.height());
    image.save(format!("target/{}.png", screen.display_info.id)).unwrap();

    image = screen.capture_area(300, 300, 300, 300).unwrap();
    println!("{}",image.height());
    image.save(format!("target/{}-2.png", screen.display_info.id)).unwrap();
  }

  let screen = Screen::from_point(100, 100).unwrap();
  println!("capturer {:?}", screen);

  let image = screen.capture_area(300, 300, 300, 300).unwrap();
  println!("{}",image.height());
  image.save("target/capture_display_with_point.png").unwrap();

  println!("运行耗时: {:?}", start.elapsed());
}
