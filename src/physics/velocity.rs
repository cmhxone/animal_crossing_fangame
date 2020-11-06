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

    // x getter
    pub fn x(&self) -> i32 {
        self.x
    }
    
    // y getter
    pub fn y(&self) -> i32 {
        self.y
    }

    // mass getter
    pub fn mass(&self) -> i32 {
        self.mass
    }

    // x setter
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    // y setter
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    // mass setter
    pub fn set_mass(&mut self, mass: i32) {
        self.mass = mass;
    }
}