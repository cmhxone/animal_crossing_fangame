#[derive(Copy, Clone, Debug)]
pub struct Velocity {
    x: i32,
    y: i32,
    mass: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32, mass: i32) -> Velocity {
        Velocity{x, y, mass}
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn mass(&self) -> i32 {
        self.mass
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_mass(&mut self, mass: i32) {
        self.mass = mass;
    }
}