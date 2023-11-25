use std::usize;
use crate::{point::Point, triangle::Triangle};

pub struct Triangulation {
    pub points: Vec<Point>,
    pub triangles: Vec<Triangle>,
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
           let triangle = triangle.unwrap().clone(); // Clonning because of mutability on self
           self.insert_point_in_triangle(i, &triangle);
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
        self.triangles.push(triangle);
    }

    fn sort_points_by_bins(&mut self) {
        let total_bins_side = (self.points.len() as f64).sqrt() as usize;  
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

    fn get_triangle_containing_point(&self, point: &Point) -> Option<&Triangle> {
        for tri in &self.triangles {
            if tri.is_point_inside_or_in_border(point, &self.points) {
                return Some(tri);
            }
        }
        None
    }


    /// Insert a point in a triangle, creating 3 new triangles and removing the old one
    /// It is assumed that the point is inside the triangle
    fn insert_point_in_triangle(&mut self, point: usize, triangle: &Triangle) {
        let p1 = triangle.p1;
        let p2 = triangle.p2;
        let p3 = triangle.p3;
        let t1 = Triangle::new_unsorted(p1, p2, point, &self.points);
        let t2 = Triangle::new_unsorted(p2, p3, point, &self.points);
        let t3 = Triangle::new_unsorted(p3, p1, point, &self.points);
        self.triangles.retain(|t| t != triangle);
        self.triangles.push(t1);
        self.triangles.push(t2);
        self.triangles.push(t3);
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
        let super_triangle = triangulation.triangles.first().unwrap().clone();
        triangulation.insert_point_in_triangle(0, &super_triangle); 

        assert_eq!(triangulation.triangles.len(), 3);

        assert_eq!(triangulation.triangles[0].p1, 2); // (1000000, 1000000)
        assert_eq!(triangulation.triangles[0].p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[0].p3, 3); // (0, -1000000)

        assert_eq!(triangulation.triangles[1].p1, 1); // (-1000000, 1000000)
        assert_eq!(triangulation.triangles[1].p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[1].p3, 3); // (0, -1000000)

        assert_eq!(triangulation.triangles[2].p1, 2); // (1000000, 1000000)
        assert_eq!(triangulation.triangles[2].p2, 0); // (0, 0)
        assert_eq!(triangulation.triangles[2].p3, 1); // (-1000000, 1000000)
    }
}
