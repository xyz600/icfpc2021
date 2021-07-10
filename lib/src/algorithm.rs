use crate::data::{Hole, Line, Point, Triangle};

const EPS: f64 = 1e-8;

pub fn next_permutation(vec: &mut Vec<usize>) -> bool {
    for i in (0..vec.len() - 1).rev() {
        if vec[i] < vec[i + 1] {
            for j in (i + 1..vec.len()).rev() {
                if vec[i] < vec[j] {
                    vec.swap(i, j);
                    vec[i + 1..].reverse();
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn test_next_permutation() {
    let mut vec = vec![0, 1, 2, 3];
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 1, 3, 2], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 2, 1, 3], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 2, 3, 1], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 3, 1, 2], vec);
    assert!(next_permutation(&mut vec));
    assert_eq!(vec![0, 3, 2, 1], vec);
}

pub fn next_duplicated_permutation(state: &mut Vec<usize>, max_value: usize) -> bool {
    for i in 0..state.len() {
        if state[i] != max_value {
            state[i] += 1;
            for j in 0..i {
                state[j] = 0;
            }
            return true;
        }
    }
    false
}

#[test]
fn test_next_duplicated_permutation() {
    let mut vec = vec![0, 0, 0, 0];
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![1, 0, 0, 0], vec);
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![2, 0, 0, 0], vec);
    assert!(next_duplicated_permutation(&mut vec, 2));
    assert_eq!(vec![0, 1, 0, 0], vec);
}

pub struct HoleDistanceCalculator {
    decomposed_triangles: Vec<Triangle>,
}

impl HoleDistanceCalculator {
    fn decompose(vertices: &Vec<Point>) -> Vec<Triangle> {
        // https://sonson.jp/blog/2007/02/12/1/

        let mut ans: Vec<Triangle> = vec![];
        let mut indices = (0..vertices.len()).collect::<Vec<usize>>();

        let find_farthest_index = |indices: &Vec<usize>| -> usize {
            let mut max_dist = 0.0;
            let mut ret = 0;
            for j in 0..indices.len() {
                let norm = vertices[indices[j]].norm();
                if max_dist < norm {
                    max_dist = norm;
                    ret = j;
                }
            }
            ret
        };

        let contain_triangle = |tri: &Triangle, v0: usize, v1: usize, v2: usize| -> bool {
            for i in 0..vertices.len() {
                let ccw_01 = Point::ccw(&tri.v0, &tri.v1, &vertices[i]);
                let ccw_12 = Point::ccw(&tri.v1, &tri.v2, &vertices[i]);
                let ccw_20 = Point::ccw(&tri.v2, &tri.v0, &vertices[i]);
                if i != v0 && i != v1 && i != v2 && ccw_01 * ccw_12 >= 0 && ccw_12 * ccw_20 >= 0 {
                    return true;
                }
            }
            false
        };
        while indices.len() >= 3 {
            // 原点から最も遠い頂点
            let i = find_farthest_index(&indices);
            let i0 = if i == 0 { indices.len() - 1 } else { i - 1 };
            let i1 = i;
            let i2 = if i == indices.len() - 1 { 0 } else { i + 1 };
            let v0 = indices[i0];
            let v1 = indices[i1];
            let v2 = indices[i2];

            let tri = Triangle::new(vertices[v0], vertices[v1], vertices[v2]);

            let base_dir = Point::ccw(&tri.v0, &tri.v1, &tri.v2);

            if !contain_triangle(&tri, v0, v1, v2) {
                ans.push(tri);
                indices.remove(i);

                continue;
            }

            let mut i = i;
            loop {
                i = if i == indices.len() - 1 { 0 } else { i + 1 };
                let i0 = if i == 0 { indices.len() - 1 } else { i - 1 };
                let i1 = i;
                let i2 = if i == indices.len() - 1 { 0 } else { i + 1 };
                let v0 = indices[i0];
                let v1 = indices[i1];
                let v2 = indices[i2];

                let tri = Triangle::new(vertices[v0], vertices[v1], vertices[v2]);
                let dir = Point::ccw(&tri.v0, &tri.v1, &tri.v2);
                if base_dir * dir > 0 && !contain_triangle(&tri, v0, v1, v2) {
                    ans.push(tri);
                    indices.remove(i);
                    break;
                }
            }
        }
        for &tri in ans.iter() {
            println!(
                "{} {} {} {} {} {}",
                tri.v0.x, tri.v0.y, tri.v1.x, tri.v1.y, tri.v2.x, tri.v2.y
            );
        }
        ans
    }

    pub fn new(hole: &Hole) -> HoleDistanceCalculator {
        HoleDistanceCalculator {
            decomposed_triangles: HoleDistanceCalculator::decompose(&hole.vertices),
        }
    }

    pub fn distance(&self, p: &Point) -> f64 {
        let mut min_distance = 1e10f64;
        for tri in self.decomposed_triangles.iter() {
            min_distance = min_distance.min(tri.distance_of(&p));
        }
        min_distance
    }
}

#[test]
fn test_triangle_decompse() {
    let mut hole = Hole::new();
    hole.push(Point::new(0.0, 0.0));
    hole.push(Point::new(1.0, 0.0));
    hole.push(Point::new(1.0, 1.0));
    hole.push(Point::new(0.0, 1.0));
    let hdc = HoleDistanceCalculator::new(&hole);
    assert_eq!(hdc.decomposed_triangles.len(), 2);
}

#[test]
fn test_triangle_decompse2() {
    let mut hole = Hole::new();
    hole.push(Point::new(0.0, 1.0));
    hole.push(Point::new(1.0, 1.0));
    hole.push(Point::new(1.0, 0.0));
    hole.push(Point::new(2.0, 0.0));
    hole.push(Point::new(2.0, 1.0));
    hole.push(Point::new(3.0, 1.0));
    hole.push(Point::new(3.0, 2.0));
    hole.push(Point::new(2.0, 2.0));
    hole.push(Point::new(2.0, 3.0));
    hole.push(Point::new(1.0, 3.0));
    hole.push(Point::new(1.0, 2.0));
    hole.push(Point::new(0.0, 2.0));
    let hdc = HoleDistanceCalculator::new(&hole);
    assert_eq!(hdc.decomposed_triangles.len(), 10);

    for tri in hdc.decomposed_triangles.iter() {
        println!(
            "{} {} {} {} {} {}",
            tri.v0.x, tri.v0.y, tri.v1.x, tri.v1.y, tri.v2.x, tri.v2.y
        )
    }
}

#[test]
fn test_hole_distance() {
    let mut hole = Hole::new();
    hole.push(Point::new(0.0, 0.0));
    hole.push(Point::new(1.0, 0.0));
    hole.push(Point::new(1.0, 1.0));
    hole.push(Point::new(0.0, 1.0));
    let hdc = HoleDistanceCalculator::new(&hole);

    let p = Point::new(-1.0, 0.0);
    assert!((hdc.distance(&p) - 1.0f64.sqrt()).abs() < EPS);
}
