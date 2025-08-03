use crate::real_vector::RealVector;

pub struct Particle {
    pub position: RealVector,
    pub velocity: RealVector,
    pub min_speed: f64,
    pub max_speed: f64,
}

impl Particle {
    pub fn new(
        position: RealVector,
        velocity: RealVector,
        min_speed: f64,
        max_speed: f64,
    ) -> Particle {
        Particle {
            position,
            velocity,
            min_speed,
            max_speed,
        }
    }

    pub fn apply_force(&mut self, force: &RealVector) {
        self.velocity = self
            .velocity
            .add(force)
            .limit(self.min_speed, self.max_speed);
    }

    pub fn update(&mut self, width: i32, height: i32) {
        self.position = self.position.add(&self.velocity);
        self.position = self.position.bring_to_box(width, height);
    }
}
