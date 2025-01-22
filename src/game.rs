use macroquad::prelude::*;

use crate::{
    ball::Ball,
    block::{Block, BLOCK_SIZE},
    player::Player,
};

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
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

pub struct Game {
    score: u32,
    player_lives: u8,
    player: Player,
    balls: Vec<Ball>,
    blocks: Vec<Block>,
}

impl Default for Game {
    fn default() -> Self {
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

        Self {
            score: 0,
            player_lives: 3,
            player: Player::new(),
            balls: vec![Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5))],
            blocks,
        }
    }
}

impl Game {
    pub async fn start(&mut self) {
        loop {
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

            self.draw();

            next_frame().await;
        }
    }

    fn draw(&self) {
        clear_background(WHITE);
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
