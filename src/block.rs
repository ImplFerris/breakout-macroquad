use macroquad::prelude::*;
pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100.0, 40.0]);

#[derive(PartialEq)]
pub enum Blocktype {
    Regular,
    SpawnBallOnDeath,
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub block_type: Blocktype,
}

impl Block {
    pub fn new(pos: Vec2, block_type: Blocktype) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            Blocktype::Regular => match self.lives {
                2 => RED,
                _ => ORANGE,
            },
            Blocktype::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
