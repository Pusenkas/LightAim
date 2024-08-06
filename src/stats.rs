use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Stats {
    pub hit: i32,
    pub shot: i32,
    pub possible: i32,
}

impl Stats {
    fn add_hit(&mut self) {
        self.hit += 1;
        self.shot += 1;
    }

    fn add_miss(&mut self) {
        self.shot += 1;
    }

    pub fn update(&mut self, res: bool) {
        if res {
            self.add_hit();
        } else {
            self.add_miss();
        }
    }

    pub fn add_possible(&mut self) {
        self.possible += 1;
    }

    pub fn get_acc(&self) -> i32 {
        if self.shot != 0 {
            100 * self.hit / self.shot
        } else {
            100
        }
    }

    pub fn get_hit_precentage(&self) -> i32 {
        if self.shot != 0 {
            100 * self.hit / self.possible
        } else {
            100
        }
    }
}
