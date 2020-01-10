extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new(
        "Mouse Draw - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let (mut width, mut height) = (WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
      

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&buffer, width / 2, height / 2)
            .unwrap();
    }
}
