use bevy::prelude::*;
use rand::thread_rng;
use rand::prelude::SliceRandom;

use super::GRID_SIZE;

#[derive(Resource)]
pub struct Positions {
    pos: Vec<(f32, f32, f32)>,
}

impl Positions {
    fn default_positions() -> Vec<(f32, f32, f32)> {
        let mut tmp = Vec::with_capacity((GRID_SIZE * GRID_SIZE) as usize);
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                tmp.push((-3.0 + 2.0 * i as f32, 8.0 + 2.0 * j as f32, -20.0));
            }
        }
        tmp
    }

    pub fn fill(&mut self) {
        self.pos = Positions::default_positions();
        self.pos.shuffle(&mut thread_rng());
    }

    pub fn inf_pop(&mut self) -> (f32, f32, f32) {
        match self.pos.pop() {
            Some(x) => x,
            _ => {
                self.fill();
                self.pos.pop().unwrap()
            }
        }
    }
}

impl Default for Positions {
    fn default() -> Self {
        let mut tmp = Positions::default_positions();
        tmp.shuffle(&mut thread_rng());
        Positions { pos: tmp }
    }
}