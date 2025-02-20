use raylib::prelude::*;

// compile time constants
const SCREEN_WIDTH: i32 = 900;
const SCREEN_HEIGHT: i32 = 800;
const COLOR_BG: Color = Color::new(0x18, 0x18, 0x18, 0xFF);

const PLAYER_HEIGHT: f32 = 10.0;
const PLAYER_WIDTH: f32 = 150.0;
const PLAYER_SPEED: f32 = 450.0;
const PLAYER_LIVES: i32 = 3;

const BALL_RADIUS: f32 = 20.0;
const BALL_SPEED: f32 = PLAYER_SPEED * 1.4;

const BRICK_WIDTH: f32 = SCREEN_WIDTH as f32 / 12.0;
const BRICK_HEIGHT: f32 = PLAYER_HEIGHT * 2.5;

struct Player {
    lives: i32,
    rect: Rectangle,
    color: Color,
    vel_x: f32,
}

struct Ball {
    vel: Vector2,
    pos: Vector2,
    color: Color,
    active: bool,
}

#[derive(Clone, Copy)]
struct Brick {
    pos: Vector2,
    color: Color,
    score: i32,
}

struct GameState {
    player: Player,
    ball: Ball,
    bricks: Vec<Brick>,
    score: i32,
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
            color: Color::ORANGE,
            vel_x: 0.0,
            lives: PLAYER_LIVES,
        }
    }
}

impl Ball {
    // collide with bricks
    // on collision pop brick and return score
    fn collide_with_bricks(&mut self, bricks: &mut Vec<Brick>) -> (i32, bool) {
        for (i, brick) in bricks.iter().enumerate() {
            let brick_rect = Rectangle {
                x: brick.pos.x,
                y: brick.pos.y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
            };

            // inside brick
            if brick_rect.check_collision_circle_rec(self.pos, BALL_RADIUS) {
                let (x, y) = (self.pos.x, self.pos.y);

                let min_dist_x = if x - brick_rect.x < brick_rect.x + brick_rect.width - x {
                    self.pos.x = brick_rect.x - BALL_RADIUS;
                    x - brick_rect.x
                } else {
                    self.pos.x = brick_rect.x + brick_rect.width + BALL_RADIUS;
                    brick_rect.x + brick_rect.width - x
                };

                let min_dist_y = if y - brick_rect.y < brick_rect.y + brick_rect.height - y {
                    self.pos.y = brick_rect.y - BALL_RADIUS;
                    y - brick_rect.y
                } else {
                    self.pos.y = brick_rect.y + brick_rect.height + BALL_RADIUS;
                    brick_rect.y + brick_rect.height - y
                };

                if min_dist_x < min_dist_y {
                    self.pos.y = y;
                    self.vel.x *= -1.0;
                } else {
                    self.pos.x = x;
                    self.vel.y *= -1.0;
                }
                let brick_score = brick.score;
                bricks.remove(i);
                let game_win = bricks.len() == 0;
                return (brick_score, game_win);
            }
        }

        (0, false)
    }

    fn init(player: &Player) -> Ball {
        Ball {
            pos: Vector2::new(
                player.rect.x + player.rect.width / 2.0,
                player.rect.y - BALL_RADIUS,
            ),
            vel: Vector2::zero(),
            color: Color::RED,
            active: false,
        }
    }
}

fn show_game_over(d: &mut RaylibDrawHandle, text: &str) -> bool {
    d.clear_background(COLOR_BG);

    let font_size = 48;
    let text_width = d.measure_text(text, font_size);

    // text
    let x = (SCREEN_WIDTH - text_width) / 2;
    let y = (SCREEN_HEIGHT - font_size) / 2;
    d.draw_text(text, x, y, font_size, Color::RAYWHITE);

    if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
        return true;
    }

    false
}

const BRICK_ROWS: usize = 6usize;
const BRICK_COLS: usize = 8usize;
fn init_bricks() -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    const PADDING: f32 = 20.0;
    let start_x = (SCREEN_WIDTH as f32 - BRICK_COLS as f32 * (BRICK_WIDTH + PADDING)) / 2.0;
    for row in 0..BRICK_ROWS {
        let color = Color::BLUE.brightness(row as f32 / BRICK_ROWS as f32);
        for col in 0..BRICK_COLS {
            bricks.push(Brick {
                pos: Vector2::new(
                    start_x + col as f32 * (PADDING + BRICK_WIDTH),
                    PADDING + row as f32 * (PADDING + BRICK_HEIGHT),
                ),
                color,
                score: 10 + (row as i32) * 10,
            });
        }
    }

    bricks
}

fn update_game(rl: &mut RaylibHandle, state: &mut GameState, dt: f32) -> bool {
    let ball = &mut state.ball;
    let player = &mut state.player;

    if !ball.active {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            ball.vel.y = -BALL_SPEED;
            ball.active = true;
        }
        ball.vel.x = player.vel_x;
    }

    // player collision
    let (x_min, x_max) = (player.rect.x, player.rect.x + player.rect.width);
    if x_min < ball.pos.x + BALL_RADIUS && ball.pos.x - BALL_RADIUS < x_max {
        let (y_min, y_max) = (player.rect.y, player.rect.y + player.rect.height);
        if y_min < ball.pos.y + BALL_RADIUS && ball.pos.y - BALL_RADIUS < y_max {
            ball.pos.y = y_min - BALL_RADIUS;
            ball.vel.y = -ball.vel.y.abs();
            ball.vel.x += 0.3 * player.vel_x;
        }
    }

    // Bounds collision
    if ball.vel.x < 0.0 && ball.pos.x - BALL_RADIUS < 0.0
        || ball.vel.x > 0.0 && ball.pos.x + BALL_RADIUS > SCREEN_WIDTH as f32
    {
        ball.vel.x *= -1.0;
    }
    if ball.vel.y < 0.0 && ball.pos.y - BALL_RADIUS < 0.0 {
        ball.vel.y *= -1.0;
    } else if ball.vel.y > 0.0 && ball.pos.y + BALL_RADIUS > SCREEN_HEIGHT as f32 {
        // Hit bottom
        let lives = player.lives - 1;
        if lives == 0 {
            player.lives = lives;
            return true;
        }

        // reset
        *player = Player::init();
        *ball = Ball::init(player);

        player.lives = lives;
    }

    let (brick_score, game_win) = ball.collide_with_bricks(&mut state.bricks);
    state.score += brick_score;

    // update pos
    ball.vel.normalize();
    ball.vel.scale(BALL_SPEED);
    ball.pos += ball.vel * dt;
    game_win
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Breaker")
        .build();
    rl.set_window_monitor(0); // stupid shit

    // game initialize
    let mut game_over = false;
    let player = Player::init();
    let ball = Ball::init(&player);
    let bricks = init_bricks();

    let mut state = GameState {
        player,
        ball,
        bricks,
        score: 0,
    };

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if game_over {
            let mut d = rl.begin_drawing(&thread);
            let text = if state.player.lives == 0 {
                "SKILL ISSUE!"
            } else if state.player.lives == 3 {
                "GOD MODE!"
            } else {
                "YOU WIN!"
            };
            if show_game_over(&mut d, text) {
                game_over = false;
                state.player = Player::init();
                state.ball = Ball::init(&state.player);
                state.bricks = init_bricks();
                state.score = 0;
            }
            continue;
        }

        state.player.update(&mut rl, dt);
        game_over = update_game(&mut rl, &mut state, dt);

        // Drawing
        let mut d = rl.begin_drawing(&thread); // d now has mut borrow of rl handle
        d.clear_background(COLOR_BG);

        // score
        let font_size = 32;
        let text = format!("Score: {}", state.score);
        let text_width = d.measure_text(&text, font_size);
        let x = (SCREEN_WIDTH - text_width) / 2;
        let y = (SCREEN_HEIGHT - font_size as i32) / 2;
        d.draw_text(&text, x, y, font_size, Color::GRAY);

        // bricks
        for brick in state.bricks.iter() {
            d.draw_rectangle_v(
                brick.pos,
                Vector2::new(BRICK_WIDTH, BRICK_HEIGHT),
                brick.color,
            );
        }

        // lives
        let life_rad = 8;
        let start_x = 10 + life_rad;
        let y = 10 + life_rad;
        for i in 0..state.player.lives {
            d.draw_circle(
                start_x + i * (2 * life_rad + 5),
                y,
                life_rad as f32,
                Color::RED,
            );
        }

        d.draw_rectangle_rec(state.player.rect, state.player.color);
        d.draw_circle_v(state.ball.pos, BALL_RADIUS, state.ball.color);
    }
}
