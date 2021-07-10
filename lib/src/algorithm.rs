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
        let mut ans = vec![];
        let mut indices = (0..vertices.len()).collect::<Vec<usize>>();

        let intersect_others = |tri: &Triangle| -> bool {
            let eye_line = Line::new(tri.v0, tri.v2);
            for i in 0..vertices.len() {
                let v0 = i;
                let v1 = if i == vertices.len() - 1 { 0 } else { i + 1 };
                let line = Line::new(vertices[v0], vertices[v1]);
                if line.intersect(&eye_line) && tri.contains_self() {
                    return true;
                }
            }
            false
        };

        let mut i = 0;
        let mut counter = 0;
        while indices.len() >= 3 {
            let v0 = if i == 0 { indices.len() - 1 } else { i - 1 };
            let v1 = i;
            let v2 = if i == indices.len() - 1 { 0 } else { i + 1 };
            let tri = Triangle::new(
                vertices[indices[v0]],
                vertices[indices[v1]],
                vertices[indices[v2]],
            );
            if !intersect_others(&tri) {
                // 耳なので取り除ける
                ans.push(tri);
                indices.remove(i);
                counter = 0;
            } else {
                counter += 1;
                i += 1;
            }
            if i == indices.len() {
                i = 0;
            }
            if counter >= 10000 {
                panic!("many loops occured during triangle-decomposition");
            }
        }
        // for tri in ans.iter() {
        //     println!(
        //         "{} {} {} {} {} {}",
        //         tri.v0.x, tri.v0.y, tri.v1.x, tri.v1.y, tri.v2.x, tri.v2.y
        //     );
        // }
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
