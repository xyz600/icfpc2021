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
    pub decomposed_triangles: Vec<Triangle>,
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

#[derive(Clone, Copy)]
struct AngleRangeInfo {
    start_index: usize,
    end_index: usize,
    start_angle: f64,
    end_angle: f64,
    start_pos: Point,
    end_pos: Point,
}

impl AngleRangeInfo {
    fn new() -> AngleRangeInfo {
        AngleRangeInfo {
            start_index: 0,
            end_index: 0,
            start_angle: 0.0,
            end_angle: 0.0,
            start_pos: Point::new(0.0, 0.0),
            end_pos: Point::new(0.0, 0.0),
        }
    }
}

struct AngleManager {
    angle_list: Vec<f64>,
    distance_list: Vec<f64>,
    vertices: Vec<Point>,
    angle_range: Vec<AngleRangeInfo>,
    base: Point,
    base_index: usize,
}

impl AngleManager {
    fn new(vertices: &Vec<Point>, base: &Point) -> AngleManager {
        let mut manager = AngleManager {
            angle_list: vec![],
            vertices: vertices.clone(),
            distance_list: vec![],
            angle_range: vec![],
            base: *base,
            base_index: 0,
        };
        for i in 0..vertices.len() {
            manager
                .angle_list
                .push(AngleManager::calculate_angle(vertices[i] - *base));
            manager.distance_list.push(vertices[i].distance(base));
        }
        manager
    }

    fn calculate_angle(v: Point) -> f64 {
        let v = v.normalize();
        if v.y >= 0.0 {
            v.x.acos()
        } else {
            std::f64::consts::PI + v.x.acos()
        }
    }

    fn normalize(&mut self, first_index: usize) {
        // 最初の index を偏角 0 にしたい
        for i in 0..self.angle_list.len() {
            self.angle_list[i] -= self.angle_list[first_index];
            if self.angle_list[i] < 0.0 {
                self.angle_list[i] += 2.0 * std::f64::consts::PI;
            }
        }
        self.base_index = first_index;
    }

    fn detect_range(&self, angle: f64) -> Option<usize> {
        // 区間に range が同じものが含まれる場合、スキップされる
        for i in 0..self.angle_range.len() {
            if self.angle_range[i].start_angle <= angle && angle < self.angle_range[i].end_angle {
                return Some(i);
            }
        }
        None
    }

    fn angle(&self, idx: usize) -> f64 {
        self.angle_list[idx]
    }

    fn distance(&self, idx: usize) -> f64 {
        self.distance_list[idx]
    }

    fn current_distance(&self, angle: f64) -> Option<f64> {
        if let Some(index) = self.detect_range(angle) {
            let n = self.angle_list.len();

            let arc = &self.angle_range[index];
            let max_dist =
                self.distance_list[arc.start_index].max(self.distance_list[arc.end_index]);

            let mut p = self.base;
            p.y += angle.sin() * max_dist;
            p.x += angle.cos() * max_dist;

            let l0 = Line::new(self.base, p);
            let l1 = Line::new(self.vertices[arc.start_index], self.vertices[arc.end_index]);
            if let Some(v) = l1.intersect_point(&l0) {
                Some(v.distance(&self.base))
            } else {
                None
            }
        } else {
            None
        }
    }

    // angle の地点で区間を分割し、分割後の小さい index を返す
    fn split_range(&mut self, angle: f64) -> usize {
        let dist = self.current_distance(angle).unwrap();

        let idx = self.detect_range(angle).unwrap();
        self.angle_range.insert(idx, self.angle_range[idx].clone());
        let nidx = idx + 1;
        let mut new_point = self.base;
        new_point.y += angle.sin() * dist;
        new_point.x += angle.cos() * dist;
        self.angle_range[idx].end_angle = angle;
        self.angle_range[idx].end_pos = new_point;
        self.angle_range[nidx].start_angle = angle;
        self.angle_range[nidx].start_pos = new_point;

        idx
    }

    fn merge(&mut self) {
        let mut index = 0;
        while index + 1 < self.angle_range.len() {
            let nindex = index + 1;
            if self.angle_range[index].start_index == self.angle_range[nindex].start_index
                && self.angle_range[index].end_index == self.angle_range[nindex].end_index
            {
                self.angle_range[nindex].start_angle = self.angle_range[index].start_angle;
                self.angle_range[nindex].start_pos = self.angle_range[index].start_pos;
                self.angle_range.remove(index);
            } else {
                index += 1;
            }
        }
    }

    fn push(&mut self, idx: usize) {
        let n = self.vertices.len();
        let next = |i: usize| -> usize { (i + 1) % n };
        let prev = |i: usize| -> usize { (i + n - 1) % n };

        let nidx = next(idx);

        let sa = self.angle(idx);
        let mut ea = self.angle(nidx);

        if nidx == self.base_index {
            ea += std::f64::consts::PI * 2.0;
        }
        let ea = ea;

        let min_a = sa.min(ea);
        let max_a = sa.max(ea);

        let min_idx = if min_a == sa { idx } else { nidx };
        let max_idx = if max_a == sa { idx } else { nidx };

        if self.largest_angle() <= min_a {
            let mut info = AngleRangeInfo::new();
            info.start_index = min_idx;
            info.start_angle = min_a;
            info.start_pos = self.vertices[min_idx];
            info.end_index = max_idx;
            info.end_angle = max_a;
            info.end_pos = self.vertices[max_idx];
            self.angle_range.push(info);
        } else {
            // 開始 index が中途半端なら merge
            let mut start_index = self.detect_range(min_a).unwrap();
            if self.angle_range[start_index].start_angle < min_a
                && min_a < self.angle_range[start_index].end_angle
            {
                self.split_range(min_a);
                start_index += 1;
            }

            let end_index = self.detect_range(max_a).unwrap();
            if self.angle_range[end_index].start_angle < max_a
                && max_a < self.angle_range[end_index].end_angle
            {
                self.split_range(max_a);
                // 開始 index はずれるけど、終了 index はずれない
            }

            let mut split_offset = 0;

            // 引っかかる全ての区間が対象
            for i in start_index..=end_index {
                let i = i + split_offset;

                let osa = self.angle_range[i].start_angle;
                let oea = self.angle_range[i].end_angle;
                let orig_start_dist = self.current_distance(osa).unwrap();
                let orig_end_dist = self.current_distance(oea).unwrap();
                let new_start_dist = self.distance(min_idx);
                let new_end_dist = self.distance(max_idx);

                if new_start_dist < orig_start_dist && new_end_dist < orig_end_dist {
                    // 最初も最後も近ければ、既存のものを全て書き換え
                    self.angle_range[i].start_index = min_idx;
                    self.angle_range[i].start_pos = self.vertices[min_idx];
                    self.angle_range[i].end_index = max_idx;
                    self.angle_range[i].end_pos = self.vertices[max_idx];
                } else if new_start_dist >= orig_start_dist && new_end_dist >= orig_end_dist {
                    // 最初も最後も遠ければ、何もしない
                } else {
                    let p0 = self.vertices[min_idx];
                    let p1 = self.vertices[max_idx];
                    let p2 = self.angle_range[i].start_pos;
                    let p3 = self.angle_range[i].end_pos;

                    let l0 = Line::new(p0, p1);
                    let l1 = Line::new(p2, p3);

                    if let Some(v) = l0.intersect_point(&l1) {
                        let angle = AngleManager::calculate_angle(v);
                        self.split_range(angle);
                        split_offset += 1;

                        if new_start_dist < orig_start_dist && new_end_dist >= orig_end_dist {
                            // 最後だけ近い場合は、その交点を見つけてそこで区間を分割
                            self.angle_range[i].start_index = min_idx;
                            self.angle_range[i].start_pos = self.vertices[min_idx];
                            self.angle_range[i].end_index = max_idx;
                            self.angle_range[i].end_pos = self.vertices[max_idx];
                        } else if new_start_dist >= orig_start_dist && new_end_dist < orig_end_dist
                        {
                            // 最後だけ近い場合は、その交点を見つけてそこで区間を分割
                            self.angle_range[i + 1].start_index = min_idx;
                            self.angle_range[i + 1].start_pos = self.vertices[min_idx];
                            self.angle_range[i + 1].end_index = max_idx;
                            self.angle_range[i + 1].end_pos = self.vertices[max_idx];
                        }
                    } else {
                        panic!();
                    }
                }
            }
            self.merge();
        }
    }

    fn largest_angle(&self) -> f64 {
        if let Some(arc) = self.angle_range.last() {
            arc.end_angle
        } else {
            0.0
        }
    }

    pub fn doIt(&mut self) -> Vec<Point> {
        // 点列が多角形を構成している
        // ここから、可視領域を示す多角形を返す

        let n = self.vertices.len();

        // 最初に選択する index
        let first_idx = {
            let mut idx = 0;
            let mut min_dist = std::f64::MAX;
            for i in 0..n {
                let dist = self.base.distance(&self.vertices[i]);
                if dist < min_dist {
                    min_dist = dist;
                    idx = i;
                }
            }
            idx
        };

        for i in 0..n {
            self.push((first_idx + i) % n);
        }

        let mut ans = vec![];
        for info in self.angle_range.iter() {
            ans.push(info.start_pos);
        }
        ans
    }
}

#[test]
fn test_visible_area() {}
