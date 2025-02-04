use macroquad::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::from_array([150.0, 40.0]);
pub const PLAYER_SPEED: f32 = 700.0;

pub struct Player {
    pub rect: Rect,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - PLAYER_SIZE.x * 0.5,
                screen_height() - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }
}

impl Player {
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move: f32 = 0.0;
        if is_key_down(KeyCode::Left) {
            x_move -= 1.;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1.;
        }

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
}
