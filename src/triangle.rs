use std::rc::Rc;
use std::cell::RefCell;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    // Uses indices to refer to points
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,

    pub n12: Option<Rc<RefCell<Triangle>>>,
    pub n23: Option<Rc<RefCell<Triangle>>>,
    pub n31: Option<Rc<RefCell<Triangle>>>,
}

impl Triangle {
    /// Create a new triangle from 3 points
    /// p1, p2, p3 are indices to points
    pub fn new(p1: usize, p2: usize, p3: usize) -> Self {
        Triangle {
            p1,
            p2,
            p3,
            n12: None,
            n23: None,
            n31: None,
        }
    }

    /// Create a new triangle from 3 points
    /// Points will be sorted using the following criteria
    /// - p1 will be the point with the greatest y value. If two points have the same y value, the one with the greatest x value will be chosen
    /// - p2 will be the one with the greatest x value among the other two
    /// - p3 will be the one with the smallest x value among the other two
    pub fn new_unsorted(p1: usize, p2: usize, p3: usize, points: &Vec<Point>) -> Self {
        let sorted = Triangle::get_sorted(p1, p2, p3, points);
        Triangle::new(sorted.0, sorted.1, sorted.2)
    }
    
    /// Returns a new instance of a Point which is the circumcenter of the triangle
    /// Requires the point vector to be passed in as an argument
    pub fn find_circumcenter(&self, points: &Vec<Rc<Point>>) -> Point {
        let p1 = &points[self.p1];
        let p2 = &points[self.p2];
        let p3 = &points[self.p3];

        let d = 2.0 * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) +
                   p3.x * (p1.y - p2.y));

        let u_x = ((p1.x.powi(2) + p1.y.powi(2)) * (p2.y - p3.y) +
               (p2.x.powi(2) + p2.y.powi(2)) * (p3.y - p1.y) +
               (p3.x.powi(2) + p3.y.powi(2)) * (p1.y - p2.y)) / d;
        let u_y = ((p1.x.powi(2) + p1.y.powi(2)) * (p3.x - p2.x) +
               (p2.x.powi(2) + p2.y.powi(2)) * (p1.x - p3.x) +
               (p3.x.powi(2) + p3.y.powi(2)) * (p2.x - p1.x)) / d;

        Point::new(u_x, u_y)
    }

    pub fn is_point_inside_or_in_border(&self, point: &Point, points: &Vec<Point>) -> bool {
        let p1 = &points[self.p1];
        let p2 = &points[self.p2];
        let p3 = &points[self.p3];
        let d1 = (point.x - p1.x) * (p2.y - p1.y) - (point.y - p1.y) * (p2.x - p1.x);
        let d2 = (point.x - p2.x) * (p3.y - p2.y) - (point.y - p2.y) * (p3.x - p2.x);
        let d3 = (point.x - p3.x) * (p1.y - p3.y) - (point.y - p3.y) * (p1.x - p3.x);

        // Check if point is in border
        if (d1 == 0.0 && d2 == 0.0) || (d2 == 0.0 && d3 == 0.0) || (d3 == 0.0 && d1 == 0.0) {
            return true;
        }

        d1 >= 0.0 && d2 >= 0.0 && d3 >= 0.0
    }

    pub fn set_neighbors(&mut self, n12: Option<Rc<RefCell<Triangle>>>, n23: Option<Rc<RefCell<Triangle>>>, n31: Option<Rc<RefCell<Triangle>>>) {
        self.n12 = n12;
        self.n23 = n23;
        self.n31 = n31;
    }

    fn get_sorted(p1: usize, p2: usize, p3: usize, points: &Vec<Point>) -> (usize, usize, usize) {
        let mut vec = vec![p1, p2, p3];
        vec.sort_by(|a, b| {
            let a_point = &points[*a];
            let b_point = &points[*b];
            // Sort by y value first and then for x value
            b_point.y.partial_cmp(&a_point.y).unwrap().then(b_point.x.partial_cmp(&a_point.x).unwrap())
        });
        let top = vec[0];
        let mut vec2 = vec![vec[1], vec[2]];
        vec2.sort_by(|a, b| {
            let a_point = &points[*a];
            let b_point = &points[*b];
            // Sort by x value, if x values are equal, sort by y value
            b_point.x.partial_cmp(&a_point.x).unwrap().then(b_point.y.partial_cmp(&a_point.y).unwrap())
        });

        (top, vec2[0], vec2[1])

    }
}
