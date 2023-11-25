use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }

    pub fn random(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min_x..max_x);
        let y = rng.gen_range(min_y..max_y);
        Self::new(x, y)
    }
}
