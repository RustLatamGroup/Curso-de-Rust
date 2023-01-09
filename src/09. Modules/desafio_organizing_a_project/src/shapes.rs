pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Circle {
    radius: f64,
}

pub enum Feature {
    Area,
    Perimeter
}

impl Rectangle{
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_circumference()
        }
    }
    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    } 

}