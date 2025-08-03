pub struct RealVector {
    pub x: f64,
    pub y: f64,
}

impl RealVector {
    pub fn add(&self, second: &RealVector) -> RealVector {
        RealVector {
            x: self.x + second.x,
            y: self.y + second.y,
        }
    }

    fn subtract(&self, second: &RealVector) -> RealVector {
        RealVector {
            x: self.x - second.x,
            y: self.y - second.y,
        }
    }

    fn multiply(&self, factor: f64) -> RealVector {
        RealVector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn limit(&self, min: f64, max: f64) -> RealVector {
        let magnitude = self.get_magnitude();

        if magnitude < 0.0001 {
            return RealVector {
                x: self.x,
                y: self.y,
            };
        }

        if magnitude > max {
            self.multiply(max / magnitude)
        } else if magnitude < min {
            self.multiply(min / magnitude)
        } else {
            RealVector {
                x: self.x,
                y: self.y,
            }
        }
    }

    fn bring_to_limits_looping(value: f64, min: f64, max: f64) -> f64 {
        let length = max - min;
        let loops = ((value - min) / length).floor();
        value - loops * length
    }

    pub fn bring_to_box(&mut self, width: i32, height: i32) -> RealVector {
        RealVector {
            x: RealVector::bring_to_limits_looping(self.x, 0.0, width as f64),
            y: RealVector::bring_to_limits_looping(self.y, 0.0, height as f64),
        }
    }

    fn get_magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    pub fn get_angle(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }
}
