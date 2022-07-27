use nalgebra::Point3;

fn main() {
let items: Vec<Point3<i32>> = vec![
    Point3::new(1, 2, 3),
    Point3::new(3, 1, 2),
    Point3::new(2, 3, 1)
];
let kdtree = kd_tree::KdTree::build(items);
let query = Point3::new(3, 1, 2);
assert_eq!(kdtree.nearest(&query).unwrap().item, &query);
}
