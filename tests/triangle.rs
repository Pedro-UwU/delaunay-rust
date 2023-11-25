use std::rc::Rc;
use delaunay::point::Point;
use delaunay::triangle::Triangle;

const EP: f64 = 0.01;

#[test]
fn test_working() {
    assert_eq!(1, 1);
}

#[test]
fn test_circumcenter() {
    // TODO add more points
    let mut points: Vec<Rc<Point>> = Vec::new();
    points.push(Rc::new(Point::new(10.0, 15.0)));
    points.push(Rc::new(Point::new(93.0, 10.0)));
    points.push(Rc::new(Point::new(-2.0, 6.0)));
    points.push(Rc::new(Point::new(0.0, 0.0)));

    let results = [(47.86, -47.98), (45.06, 18.35)];
    let mut i = 0;
    for tri_i in 0..(points.len() - 2) {
        let tri = Triangle::new(tri_i, tri_i + 1, tri_i + 2);
        let circumcenter = tri.find_circumcenter(&points);
        assert!(results[i].0 - EP <= circumcenter.x && circumcenter.x <= results[i].0 + EP);
        i += 1;
    }
}

#[test]
fn test_sorted_points() {
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0.0, 0.0));
    points.push(Point::new(10.0, 10.0));
    points.push(Point::new(20.0, 0.0));
    let t1 = Triangle::new_unsorted(0, 1, 2, &points);
    let t2 = Triangle::new_unsorted(0, 2, 1, &points);
    let t3 = Triangle::new_unsorted(1, 0, 2, &points);

    assert_eq!(t1.p1, 1);
    assert_eq!(t2.p1, 1);
    assert_eq!(t3.p1, 1);
    assert_eq!(t1.p2, 2);
    assert_eq!(t2.p2, 2);
    assert_eq!(t3.p2, 2);
    assert_eq!(t1.p3, 0);
    assert_eq!(t2.p3, 0);
    assert_eq!(t3.p3, 0);

    // now add a point with negative y
    points.push(Point::new(100.0, -10.0));
    let t4 = Triangle::new_unsorted(0, 2, 3, &points);
    assert_eq!(t4.p1, 2);
    assert_eq!(t4.p2, 3);
    assert_eq!(t4.p3, 0);

}

#[test]
fn test_point_inside() {
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0.0, 0.0));
    points.push(Point::new(10.0, 10.0));
    points.push(Point::new(20.0, 0.0));

    let tri = Triangle::new_unsorted(0, 1, 2, &points);
    let p1 = Point::new(5.0, 5.0);
    let p2 = Point::new(10.0, 5.0);
    let p3 = Point::new(20.0, 5.0);
    let p4 = Point::new(-1.0, 0.0);

    assert_eq!(tri.is_point_inside_or_in_border(&p1, &points), true);
    assert_eq!(tri.is_point_inside_or_in_border(&p2, &points), true);
    assert_eq!(tri.is_point_inside_or_in_border(&p3, &points), false);
    assert_eq!(tri.is_point_inside_or_in_border(&p4, &points), false);
}
