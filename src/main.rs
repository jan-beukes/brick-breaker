use raylib::prelude::*;

// compile time constants
const SCREEN_WIDTH: i32 = 900;
const SCREEN_HEIGHT: i32 = 800;
const COLOR_BG: Color = Color::new(0x18, 0x18, 0x18, 0xFF);

const PLAYER_HEIGHT: f32 = 30.0;
const PLAYER_WIDTH: f32 = 150.0;
const PLAYER_SPEED: f32 = 300.0;

const BALL_RADIUS: f32 = 20.0;
const BALL_SPEED: f32 = PLAYER_SPEED * 1.5;

struct Player {
    rect: Rectangle,
    color: Color,
    vel_x: f32,
}

struct Ball {
    pos: Vector2,
    vel: Vector2,
    color: Color,
    active: bool,
}

impl Player {
    // by taking &mut of rl we borrow instead of moving the handle to this fucntion
    fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.vel_x = -PLAYER_SPEED;
        } else if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.vel_x = PLAYER_SPEED;
        } else {
            self.vel_x = 0.0
        };

        self.rect.x += self.vel_x * dt;

        self.rect.x = self
            .rect
            .x
            .clamp(0.0, SCREEN_WIDTH as f32 - self.rect.width);
    }

    fn init() -> Player {
        let start_x = (SCREEN_WIDTH as f32 - PLAYER_WIDTH) / 2.0;
        let start_y = SCREEN_HEIGHT as f32 - PLAYER_HEIGHT * 2.0;
        Player {
            rect: Rectangle::new(start_x, start_y, PLAYER_WIDTH, PLAYER_HEIGHT),
            color: Color::RED,
            vel_x: 0.0,
        }
    }
}

impl Ball {
    fn update(&mut self, rl: &mut RaylibHandle, player: &Player, dt: f32) -> bool {
        if !self.active {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.vel.y = -BALL_SPEED;
                self.active = true;
            }
            self.vel.x = player.vel_x;
        }

        // player collision
        let (x_min, x_max) = (player.rect.x, player.rect.x + player.rect.width);
        if x_min < self.pos.x + BALL_RADIUS && self.pos.x - BALL_RADIUS < x_max {
            let (y_min, y_max) = (player.rect.y, player.rect.y + player.rect.height);
            if y_min < self.pos.y + BALL_RADIUS && self.pos.y - BALL_RADIUS < y_max {
                self.pos.y = y_min - BALL_RADIUS;
                self.vel.y = -self.vel.y.abs();
            }
        }

        // Bounds collision
        if self.vel.x < 0.0 && self.pos.x - BALL_RADIUS < 0.0
            || self.vel.x > 0.0 && self.pos.x + BALL_RADIUS > SCREEN_WIDTH as f32
        {
            self.vel.x *= -1.0;
        }
        if self.vel.y < 0.0 && self.pos.y - BALL_RADIUS < 0.0 {
            self.vel.y *= -1.0;
        } else if self.vel.y > 0.0 && self.pos.y + BALL_RADIUS > SCREEN_HEIGHT as f32 {
            return true;
        }

        // update pos
        self.pos += self.vel * dt;
        return false;
    }

    fn init(player: &Player) -> Ball {
        Ball {
            pos: Vector2::new(
                player.rect.x + player.rect.width / 2.0,
                player.rect.y - BALL_RADIUS,
            ),
            vel: Vector2::zero(),
            color: Color::BLUEVIOLET,
            active: false,
        }
    }
}

fn show_game_over(d: &mut RaylibDrawHandle) -> bool {
    d.clear_background(COLOR_BG);

    let font_size = 48;
    let text = "SKILL ISSUE!";
    let text_width = d.measure_text(text, font_size);

    // text
    let x = (SCREEN_WIDTH - text_width) / 2;
    let y = (SCREEN_HEIGHT - font_size) / 2;
    d.draw_text(text, x, y, font_size, Color::RAYWHITE);

    if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
        return true;
    }

    return false;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Breaker")
        .build();
    rl.set_window_monitor(0); // stupid shit

    let font_size = 38;
    let text = "AMONG US";
    let text_width = rl.measure_text(text, font_size);

    let mut player = Player::init();
    let mut ball = Ball::init(&player);
    let mut game_over = false;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if game_over {
            let mut d = rl.begin_drawing(&thread);
            if show_game_over(&mut d) {
                game_over = false;
                player = Player::init();
                ball = Ball::init(&player);
            }
            continue;
        }

        player.update(&mut rl, dt);
        game_over = ball.update(&mut rl, &player, dt);

        // Drawing

        let mut d = rl.begin_drawing(&thread); // d now has mut borrow of rl handle
        d.clear_background(COLOR_BG);

        d.draw_rectangle_rec(player.rect, player.color);
        d.draw_circle_v(ball.pos, BALL_RADIUS, ball.color);

        // text
        let x = (SCREEN_WIDTH - text_width) / 2;
        let y = (SCREEN_HEIGHT - font_size) / 2;
        d.draw_text(text, x, y, font_size, Color::NAVAJOWHITE);
    }
}
