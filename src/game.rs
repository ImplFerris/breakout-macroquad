use macroquad::prelude::*;

use crate::{
    ball::Ball,
    block::{Block, BLOCK_SIZE},
    player::Player,
};

#[derive(Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    LevelCompleted,
    Dead,
}

#[derive(Default)]
pub struct Game {
    state: GameState,
    score: u32,
    player_lives: i8,
    player: Player,
    balls: Vec<Ball>,
    blocks: Vec<Block>,
}

impl Game {
    pub fn reset_game(&mut self) {
        self.player_lives = 3;
        self.score = 0;
        self.player = Player::default();

        self.balls = vec![(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)))];

        let mut blocks = Vec::new();
        let (width, height) = (6, 6);
        let padding = 5.0;
        let total_block_size = BLOCK_SIZE + vec2(padding, padding);
        let board_start_pos = vec2(
            (screen_width() - (total_block_size.x * width as f32)) * 0.5,
            50.0,
        ) + 5.0;
        for i in 0..width * height {
            let block_x = (i % width) as f32 * total_block_size.x;
            let block_y = (i / width) as f32 * total_block_size.y;
            blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
        }
        self.blocks = blocks;
    }

    pub async fn start(&mut self) {
        self.reset_game();
        loop {
            match self.state {
                GameState::Menu => {
                    if is_key_pressed(KeyCode::Space) {
                        self.reset_game();
                        self.state = GameState::Playing;
                    }
                }
                GameState::Playing => {
                    if is_key_pressed(KeyCode::Space) {
                        self.balls
                            .push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));
                    }

                    self.player.update(get_frame_time());
                    for ball in self.balls.iter_mut() {
                        ball.update(get_frame_time());
                    }

                    self.collison_handle();
                    self.remove_balls();

                    self.blocks.retain(|block| block.lives > 0);
                    if self.blocks.is_empty() {
                        self.state = GameState::LevelCompleted;
                    }
                }
                _ => {
                    if is_key_pressed(KeyCode::Space) {
                        self.state = GameState::Menu;
                    }
                }
            }

            clear_background(WHITE);

            match self.state {
                GameState::Menu => draw_title_text("Press SPACE to start..."),
                GameState::Playing => self.draw_game(),
                GameState::LevelCompleted => {
                    draw_title_text(&format!("You win! {} score", self.score))
                }
                GameState::Dead => draw_title_text(&format!("You died! {} score", self.score)),
            }

            next_frame().await;
        }
    }

    fn draw_game(&self) {
        self.player.draw();
        for block in self.blocks.iter() {
            block.draw();
        }
        for ball in self.balls.iter() {
            ball.draw();
        }

        self.print_score();
        self.print_lives();
    }

    fn collison_handle(&mut self) {
        for ball in self.balls.iter_mut() {
            resolve_collison(&mut ball.rect, &mut ball.vel, &self.player.rect);

            for block in self.blocks.iter_mut() {
                if resolve_collison(&mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                    if block.lives <= 0 {
                        self.score += 10;
                    }
                }
            }
        }
    }

    fn remove_balls(&mut self) {
        let balls_len = self.balls.len();
        let was_last_ball = balls_len == 1;
        self.balls.retain(|ball| ball.rect.y < screen_height());
        let remove_balls = balls_len - self.balls.len();
        if remove_balls > 0 && was_last_ball {
            self.player_lives -= 1;
            if self.player_lives <= 0 {
                self.state = GameState::Dead;
            }
        }
    }

    fn print_score(&self) {
        let score_text = format!("Score: {}", self.score);
        let score_text_dim = measure_text(&score_text, None, 30, 1.0);

        draw_text_ex(
            &score_text,
            screen_width() * 0.5 - score_text_dim.width * 0.5,
            40.0,
            TextParams {
                font_size: 30,
                color: BLACK,
                ..Default::default()
            },
        );
    }

    fn print_lives(&self) {
        draw_text_ex(
            &format!("lives: {}", self.player_lives),
            30.0,
            40.0,
            TextParams {
                font_size: 30,
                color: BLACK,
                ..Default::default()
            },
        );
    }
}

fn draw_title_text(title: &str) {
    let dims = measure_text(title, None, 50, 1.0);
    draw_text_ex(
        title,
        screen_width() * 0.5 - dims.width * 0.5,
        screen_height() * 0.5 - dims.height * 0.5,
        TextParams {
            font_size: 50,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn resolve_collison(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let Some(intersection) = a.intersect(*b) else {
        return false;
    };

    let a_center = a.center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = to.signum();

    if intersection.w > intersection.h {
        a.y -= to_signum.y * intersection.h;
        match to_signum.y > 0.0 {
            true => vel.y = -vel.y.abs(),
            false => vel.y = vel.y.abs(),
        };
    } else {
        a.x -= to_signum.x * intersection.w;
        match to_signum.x > 0.0 {
            true => vel.x = vel.x.abs(),
            false => vel.x = -vel.x.abs(),
        }
    }

    true
}
