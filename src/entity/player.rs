use sdl2::rect::Rect;
use super::super::physics::velocity::Velocity;
use std::collections::HashSet;
use sdl2::keyboard::Keycode;

#[derive(Copy, Clone)]
pub struct Player {
    src_rect: Rect,
    dst_rect: Rect,
    velocity: Velocity,
    speed: u32,
}

impl Player {
    pub fn new(src: Rect, dst: Rect, velocity: Velocity) -> Player {
        Player{src_rect: src, dst_rect: dst, velocity: velocity, speed: 4}
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn src_rect(&self) -> Rect {
        self.src_rect
    }

    pub fn dst_rect(&self) -> Rect {
        self.dst_rect
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn render_rect(&self, camera: &Rect) -> Rect {
        Rect::new(
            self.dst_rect().x() - camera.x(),
            self.dst_rect().y() - camera.y(),
            self.dst_rect().width(),
            self.dst_rect().height()
        )
    }

    pub fn set_velocity(&mut self, x: i32, y: i32, mass: i32) {
        self.velocity.set_x(x);
        self.velocity.set_y(y);
        self.velocity.set_mass(mass);
    }

    pub fn set_velocity_x(&mut self, x: i32) {
        self.velocity.set_x(x);
    }

    pub fn set_velocity_y(&mut self, y: i32) {
        self.velocity.set_y(y);
    }
}