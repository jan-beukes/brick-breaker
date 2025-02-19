use raylib::prelude::*;
use std::io::prelude::*;

// compile time constants
const SCREEN_WIDTH: i32 = 900;
const SCREEN_HEIGHT: i32 = 800;
const COLOR_BG: Color = Color::new(0x18, 0x18, 0x18, 0xFF);
const PLAYER_HEIGHT: f32 = 30.0;
const PLAYER_WIDTH: f32 = 150.0;

struct Player {
    rect: Rectangle,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Breaker")
        .build();

    // test
    let path = std::path::Path::new("test.txt");
    let mut file = std::fs::File::create(path).unwrap();
    let _ = file.write("AM I COOKED".as_bytes());

    let font_size = 48;
    let text = "Hello From Rust!";
    let text_width = rl.measure_text(text, font_size);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread); // d now borrows rl handle
        d.clear_background(COLOR_BG);

        let start_x = (SCREEN_WIDTH as f32 - PLAYER_WIDTH) / 2.0;
        let start_y = SCREEN_HEIGHT as f32 - PLAYER_HEIGHT * 2.0;
        let player = Player {
            rect: Rectangle::new(start_x, start_y, PLAYER_WIDTH, PLAYER_HEIGHT),
            color: Color::RED,
        };

        d.draw_rectangle_rec(player.rect, player.color);

        // text
        let x = (SCREEN_WIDTH - text_width) / 2;
        let y = (SCREEN_HEIGHT - font_size) / 2;
        d.draw_text(text, x, y, font_size, Color::MAGENTA);
    }
}
