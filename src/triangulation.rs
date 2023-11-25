use std::usize;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{point::Point, triangle::Triangle};

pub struct Triangulation {
    pub points: Vec<Point>,
    pub triangles: Vec<Rc<RefCell<Triangle>>>,
// Use RefCell
}

impl Triangulation {
    pub fn new(points: Vec<Point>) -> Self {
        Triangulation {
            points,
            triangles: Vec::new(),
        }
    }

    pub fn triangulate(&mut self) {
        // First, sort the points by bins
        self.sort_points_by_bins();
        // First, create a giant super triangle
        self.create_giant_super_triangle();
        // For each point, insert it
        for (i, p) in self.points.clone().iter().enumerate() {
           let triangle = self.get_triangle_containing_point(p);
           if triangle.is_none() {
               panic!("Point not found in any triangle");
           }
           let triangle_index = triangle.unwrap();
           self.insert_point_in_triangle(i, triangle_index);
        }
        
    }

    fn create_giant_super_triangle(&mut self) {
        let p1 = Point::new(-1000000.0, 1000000.0);
        let p2 = Point::new(1000000.0, 1000000.0);
        let p3 = Point::new(0.0, -1000000.0);
        self.points.push(p1);
        self.points.push(p2);
        self.points.push(p3);
        let p1 = self.points.len() - 3;
        let p2 = self.points.len() - 2;
        let p3 = self.points.len() - 1;

        let triangle = Triangle::new_unsorted(p1, p2, p3, &self.points);
        self.triangles.push(Rc::new(RefCell::new(triangle)));

    }

    fn sort_points_by_bins(&mut self) {
        let total_bins_side = (self.points.len() as f64).sqrt().ceil() as usize;  
        self.points.sort_by(|a, b| {
            let mut a_x = (a.x / total_bins_side as f64).floor() as usize;
            let mut b_x = (b.x / total_bins_side as f64).floor() as usize;
            let a_y = (a.y / total_bins_side as f64).floor() as usize;
            let b_y = (b.y / total_bins_side as f64).floor() as usize;

            if a_y < b_y {
                return std::cmp::Ordering::Less;
            }
            if a_y % 2 == 0 {
                 a_x = total_bins_side - a_x + 1;
            }
            if b_y % 2 == 0 {
                b_x = total_bins_side - b_x + 1;
            }

            return (a_y * total_bins_side + a_x).cmp(&(b_y * total_bins_side + b_x))
        });
    }

    fn get_triangle_containing_point(&self, point: &Point) -> Option<usize> {
        for (i, tri) in self.triangles.iter().enumerate() {
            if tri.borrow().is_point_inside_or_in_border(point, &self.points) {
                return Some(i);
            }
        }
        None
    }


    /// Insert a point in a triangle, creating 3 new triangles and removing the old one
    /// It is assumed that the point is inside the triangle
    fn insert_point_in_triangle(&mut self, point: usize, triangle_index: usize) {
        let triangle = self.triangles[triangle_index].borrow_mut().clone();
        let p1 = triangle.p1;
        let p2 = triangle.p2;
        let p3 = triangle.p3;
        let t1 = Triangle::new_unsorted(p1, p2, point, &self.points);
        let t2 = Triangle::new_unsorted(p2, p3, point, &self.points);
        let t3 = Triangle::new_unsorted(p3, p1, point, &self.points);
        let triangle_index = self.triangles.iter().position(|x| x.borrow().p1 == triangle.p1).unwrap();
        self.triangles.remove(triangle_index);
        
        let t1 = Rc::new(RefCell::new(t1));
        let t2 = Rc::new(RefCell::new(t2));
        let t3 = Rc::new(RefCell::new(t3));

        // TODO - Manage the neighbours

        self.triangles.push(Rc::clone(&t1));
        self.triangles.push(Rc::clone(&t2));
        self.triangles.push(Rc::clone(&t3));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_point_insertion() {
        let mut points: Vec<Point> = Vec::new();
        points.push(Point::new(0.0, 0.0));
        let mut triangulation  = Triangulation::new(points);
        triangulation.create_giant_super_triangle();
        // p[0] = (0, 0)
        // p[1] = (-1000000, 1000000)
        // p[2] = (1000000, 1000000)
        // p[3] = (0, -1000000)
        triangulation.insert_point_in_triangle(0, 0); 

        assert_eq!(triangulation.triangles.len(), 3);

        assert_eq!(triangulation.triangles[0].borrow().p1, 2); // (1000000, 1000000)
        assert_eq!(triangulation.triangles[0].borrow().p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[0].borrow().p3, 3); // (0, -1000000)

        assert_eq!(triangulation.triangles[1].borrow().p1, 1); // (-1000000, 1000000)
        assert_eq!(triangulation.triangles[1].borrow().p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[1].borrow().p3, 3); // (0, -1000000)

        assert_eq!(triangulation.triangles[2].borrow().p1, 2); // (1000000, 1000000)
        assert_eq!(triangulation.triangles[2].borrow().p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[2].borrow().p3, 1); // (-1000000, 1000000)
    }

    #[test]
    fn test_point_insertion_neightbors() {
        let mut points: Vec<Point> = Vec::new();
        points.push(Point::new(0.0, 0.0));
        let mut triangulation  = Triangulation::new(points);
        triangulation.create_giant_super_triangle();
        // p[0] = (0, 0)
        // p[1] = (-1000000, 1000000)
        // p[2] = (1000000, 1000000)
        // p[3] = (0, -1000000)
        triangulation.insert_point_in_triangle(0, 0); 
        // t[0] = (2, 0, 3)
        // t[1] = (1, 0, 3)
        // t[2] = (2, 0, 1)

        // t[0].12 = t[2]
        // t[0].23 = t[1]
        // t[0].31 = None
        
        // t[1].12 = t[2]
        // t[1].23 = t[0]
        // t[1].31 = None
        
        // t[2].12 = t[0]
        // t[2].23 = t[1]
        // t[2].31 = None
    }
}
