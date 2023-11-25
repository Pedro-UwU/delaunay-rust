use delaunay::point::Point;

fn main() {
    let mut points: Vec<Point> = Vec::new();
    for _i in 0..10 {
        points.push(Point::random(0.0, 100.0, 0.0, 100.0));
    }
}
