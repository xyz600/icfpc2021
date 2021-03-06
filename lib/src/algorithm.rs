use crate::data::{Hole, Line, Point, Triangle};
use std::collections::HashSet;

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
            // ??????????????????????????????
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
            let dist = tri.distance_of(&p);
            min_distance = min_distance.min(dist);
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

// #[test]
fn test_hole_distance2() {
    let ps = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 2.0),
        Point::new(2.0, 2.0),
        Point::new(4.0, 4.0),
        Point::new(4.0, 0.0),
    ];
    let hole = Hole { vertices: ps };

    let hdc = HoleDistanceCalculator::new(&hole);

    let distance_exp = vec![
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![1.0, 1.0, 0.5f64.sqrt(), 0.0, 0.0],
        vec![2.0, 2.0, 2.0f64.sqrt(), 0.5f64.sqrt(), 0.0],
    ];

    for y in 0..=4 {
        for x in 0..4 {
            let p = Point::new(x as f64, y as f64);
            let dist = hdc.distance(&p);
            print!("{} ", dist);
        }
        println!();
    }

    let p = Point::new(2.0, 4.0);
    assert!((hdc.distance(&p) - 2.0f64.sqrt()).abs() < EPS);

    for y in 0..=4 {
        for x in 0..4 {
            let p = Point::new(x as f64, y as f64);
            let dist = hdc.distance(&p);
            assert!((dist - distance_exp[y][x]).abs() < EPS);
        }
    }
}

#[derive(Clone, Copy)]
struct Range {
    left: f64,
    right: f64,
    left_inclusive: bool,
    right_inclusive: bool,
}

impl Range {
    fn new(left: f64, right: f64, left_inclusive: bool, right_inclusive: bool) -> Range {
        Range {
            left: left,
            right: right,
            left_inclusive: left_inclusive,
            right_inclusive: right_inclusive,
        }
    }

    fn is_inside(&self, value: f64, is_inclusive: bool) -> bool {
        if is_inclusive {
            if self.left == value && self.left_inclusive {
                true
            } else if self.right == value && self.right_inclusive {
                true
            } else {
                self.left < value && value < self.right
            }
        } else {
            if self.left == value && !self.left_inclusive {
                true
            } else if self.right == value && !self.right_inclusive {
                true
            } else {
                self.left < value && value < self.right
            }
        }
    }

    fn split(&self, value: f64, left_inclusive: bool) -> (Range, Range) {
        assert!(self.is_inside(value, true));

        let left = Range::new(self.left, value, self.left_inclusive, left_inclusive);
        let right = Range::new(value, self.right, !left_inclusive, self.right_inclusive);
        (left, right)
    }

    fn merge_right(&mut self, right: &Range) {
        assert!(self.right == right.left);
        // ??????????????????????????????????????????????????????????????????
        assert!(self.right_inclusive ^ right.left_inclusive);

        self.right = right.right;
        self.right_inclusive = right.right_inclusive;
    }
}

#[derive(Clone)]
struct AngleRangeInfo {
    start_index: usize,
    end_index: usize,
    start_pos: Point,
    end_pos: Point,
    range: Range,
}

impl AngleRangeInfo {
    fn new() -> AngleRangeInfo {
        AngleRangeInfo {
            start_index: 0,
            end_index: 0,
            range: Range::new(0.0, 0.0, false, false),
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

const NONE: usize = std::usize::MAX;

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
                .push(manager.calculate_angle(vertices[i] - *base));
            manager.distance_list.push(vertices[i].distance(base));
        }
        let mut info = AngleRangeInfo::new();
        info.range.left = 0.0;
        info.range.right = std::f64::consts::PI * 2.0;
        info.range.left_inclusive = true;
        info.range.right_inclusive = false;
        info.start_index = NONE;
        info.end_index = NONE;
        manager.angle_range.push(info);

        manager
    }

    fn calculate_angle(&self, v: Point) -> f64 {
        let v = v.normalize();
        if v.y >= 0.0 {
            v.x.acos()
        } else {
            std::f64::consts::PI * 2.0 - v.x.acos()
        }
    }

    fn detect_range(&self, _angle: f64, is_inclusive: bool) -> Option<usize> {
        // [0.0, 2PI) ???????????????????????????????????????????????????????????????
        let angle = if _angle == std::f64::consts::PI * 2.0 && is_inclusive {
            0.0
        } else {
            _angle
        };
        // ????????? range ????????????????????????????????????????????????????????????
        for i in (0..self.angle_range.len()).rev() {
            if self.angle_range[i].range.is_inside(angle, is_inclusive) {
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

    fn sin2PI(&self, angle: f64) -> f64 {
        if angle < std::f64::consts::PI {
            angle.sin()
        } else {
            -angle.sin()
        }
    }

    fn cos2PI(&self, angle: f64) -> f64 {
        if angle < std::f64::consts::PI {
            angle.cos()
        } else {
            (std::f64::consts::PI * 2.0 - angle).cos()
        }
    }

    fn current_distance(&self, angle: f64, is_inclusive: bool) -> Option<f64> {
        if let Some(index) = self.detect_range(angle, is_inclusive) {
            let arc = &self.angle_range[index];
            if arc.start_index == NONE {
                // ??????????????????????????? None ????????????
                return Some(std::f64::MAX);
            }
            let max_dist =
                self.distance_list[arc.start_index].max(self.distance_list[arc.end_index]);

            let mut p = self.base;
            p.y += self.sin2PI(angle) * max_dist;
            p.x += self.cos2PI(angle) * max_dist;

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

    // angle ?????????????????????????????????????????????????????? index ?????????
    // ???????????????????????????????????? left_inclusive = true
    fn split_range(&mut self, angle: f64, left_inclusive: bool) -> usize {
        let dist = self.current_distance(angle, true).unwrap();

        let idx = self.detect_range(angle, true).unwrap();
        self.angle_range.insert(idx, self.angle_range[idx].clone());
        let nidx = idx + 1;
        let mut new_point = self.base;

        new_point.y += self.sin2PI(angle) * dist;
        new_point.x += self.cos2PI(angle) * dist;

        let (left_range, right_range) = self.angle_range[idx].range.split(angle, false);

        self.angle_range[idx].end_pos = new_point;
        self.angle_range[idx].range = left_range;
        self.angle_range[nidx].start_pos = new_point;
        self.angle_range[nidx].range = right_range;

        idx
    }

    fn merge(&mut self) {
        let mut index = 0;
        while index + 1 < self.angle_range.len() {
            let nindex = index + 1;
            if self.angle_range[index].start_index == self.angle_range[nindex].start_index
                && self.angle_range[index].end_index == self.angle_range[nindex].end_index
            {
                let nrange = self.angle_range[nindex].range;
                self.angle_range[index].range.merge_right(&nrange);
                self.angle_range[index].end_pos = self.angle_range[nindex].end_pos;
                self.angle_range.remove(index);
            } else {
                index += 1;
            }
        }
    }

    fn push_range(
        &mut self,
        min_idx: usize,
        max_idx: usize,
        min_a: f64,
        max_a: f64,
        min_a_inclusive: bool,
    ) {
        // 0?????????????????????????????????????????????

        let max_a_inclusive = !min_a_inclusive;

        // ?????? index ????????????????????? slpit
        let mut start_index = self.detect_range(min_a, min_a_inclusive).unwrap();
        if self.angle_range[start_index].range.left < min_a {
            self.split_range(min_a, !min_a_inclusive);
            start_index += 1;
        }

        let end_index = self.detect_range(max_a, max_a_inclusive).unwrap();
        if self.angle_range[end_index].range.right > max_a {
            self.split_range(max_a, max_a_inclusive);
        }

        // query ????????????????????????????????????????????????????????????

        let mut split_offset = 0;

        // ???????????????????????????????????????
        for i in start_index..=end_index {
            let i = i + split_offset;

            let osa = self.angle_range[i].range.left;
            let oea = self.angle_range[i].range.right;
            let orig_start_dist = self
                .current_distance(osa, self.angle_range[i].range.left_inclusive)
                .unwrap();
            let orig_end_dist = self
                .current_distance(oea, self.angle_range[i].range.right_inclusive)
                .unwrap();
            let new_start_dist = self.distance(min_idx);
            let new_end_dist = self.distance(max_idx);

            if new_start_dist < orig_start_dist && new_end_dist < orig_end_dist {
                // ?????????????????????????????????????????????????????????????????????
                self.angle_range[i].start_index = min_idx;
                self.angle_range[i].start_pos = self.vertices[min_idx];
                self.angle_range[i].end_index = max_idx;
                self.angle_range[i].end_pos = self.vertices[max_idx];
            } else if new_start_dist >= orig_start_dist && new_end_dist >= orig_end_dist {
                // ????????????????????????????????????????????????
            } else {
                let p0 = self.vertices[min_idx];
                let p1 = self.vertices[max_idx];
                let p2 = self.angle_range[i].start_pos;
                let p3 = self.angle_range[i].end_pos;

                let l0 = Line::new(p0, p1);
                let l1 = Line::new(p2, p3);

                if let Some(v) = l0.intersect_point(&l1) {
                    let angle = self.calculate_angle(v);
                    // ??????????????????????????????????????????????????????????????????????????????????????????????????????
                    self.split_range(angle, true);
                    split_offset += 1;

                    if new_start_dist < orig_start_dist && new_end_dist > orig_end_dist {
                        // ?????????????????????????????????????????????????????????????????????????????????
                        self.angle_range[i].start_index = min_idx;
                        self.angle_range[i].start_pos = self.vertices[min_idx];
                        self.angle_range[i].end_index = max_idx;
                        self.angle_range[i].end_pos = self.vertices[max_idx];
                    } else if new_start_dist > orig_start_dist && new_end_dist < orig_end_dist {
                        // ?????????????????????????????????????????????????????????????????????????????????
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

    fn push(&mut self, idx: usize) {
        let n = self.vertices.len();
        let next = |i: usize| -> usize { (i + 1) % n };

        let nidx = next(idx);

        const PI: f64 = std::f64::consts::PI;

        let sa = self.angle(idx);
        let ea = self.angle(nidx);

        let min_a = sa.min(ea);
        let max_a = sa.max(ea);

        let min_idx = if min_a == sa { idx } else { nidx };
        let max_idx = if max_a == sa { idx } else { nidx };

        let min_a_inclusive = min_idx == idx;

        // 180 ????????????????????????????????????0 ??????????????????????????????
        if max_a - min_a > PI {
            if 0.0 < min_a {
                self.push_range(idx, nidx, 0.0, min_a, min_a_inclusive);
            }
            if max_a < PI * 2.0 {
                self.push_range(min_idx, max_idx, max_a, PI * 2.0, min_a_inclusive);
            }
        } else {
            self.push_range(min_idx, max_idx, min_a, max_a, min_a_inclusive);
        }
    }

    pub fn doIt(&mut self) -> Vec<Point> {
        // ???????????????????????????????????????
        // ??????????????????????????????????????????????????????

        let n = self.vertices.len();

        // ????????????????????? index
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

// #[test]
fn test_visible_area() {
    let base = Point::new(1.0, 1.0);

    let ps = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 2.0),
        Point::new(2.0, 2.0),
        Point::new(2.0, 4.0),
        Point::new(4.0, 4.0),
        Point::new(4.0, 0.0),
    ];

    let mut angle_manager = AngleManager::new(&ps, &base);
    let ans = angle_manager.doIt();
}

// pub fn check_visibility(pos_list: &Vec<Point>, base: &Point) -> Vec<Point> {
//     let visible = |p: &Point| -> bool {
//         let n = pos_list.len();
//         for i in 0..pos_list.len() {
//             let p1 = pos_list[i];
//             let p2 = pos_list[(i + 1) % n];

//             let l0 = Line::new(*base, *p);
//             let l1 = Line::new(p1, p2);

//             // ???????????????????????????
//             if l0.on_same_line(&l1) {
//                 continue;
//             }

//             if base.x == p1.x && base.y == p1.y {

//             } else if base.y == p1.x

//             if  && l0.intersect(&l1) {
//                 return false;
//             }
//         }
//         true
//     };

//     let mut vec = vec![];
//     for p in pos_list.iter() {
//         if visible(p) {
//             vec.push(*p);
//         }
//     }
//     vec
// }

// #[test]
// fn test_visibility1() {
//     // ???????????????
//     let ps = vec![
//         Point::new(0.0, 0.0),
//         Point::new(0.0, 2.0),
//         Point::new(2.0, 2.0),
//         Point::new(2.0, 4.0),
//         Point::new(4.0, 4.0),
//         Point::new(4.0, 0.0),
//     ];

//     let base = Point::new(1.0, 1.0);
//     let ret = check_visibility(&ps, &base);
//     println!("{:?}", ret);
//     assert_eq!(ret.len(), 5);
// }

// #[test]
// fn test_visibility2() {
//     // ??????????????????
//     let ps = vec![
//         Point::new(0.0, 0.0),
//         Point::new(0.0, 2.0),
//         Point::new(2.0, 2.0),
//         Point::new(2.0, 4.0),
//         Point::new(4.0, 4.0),
//         Point::new(4.0, 0.0),
//     ];

//     let base = Point::new(0.0, 1.0);
//     let ret = check_visibility(&ps, &base);
//     println!("{:?}", ret);
//     assert_eq!(ret.len(), 4);
// }

// pub fn enumerate_visible_point(hole: &Hole, base: &Point) -> HashSet<i64> {
//     // ???????????????????????? x * 10000 + y ??????????????????

//     let vertices = check_visibility(&hole.vertices, base);
//     let subhole = Hole { vertices: vertices };

//     let hdc = HoleDistanceCalculator::new(&hole);

//     let sub_hdc = HoleDistanceCalculator::new(&subhole);

//     let mut min_x = std::i64::MAX;
//     let mut max_x = std::i64::MIN;
//     let mut min_y = std::i64::MAX;
//     let mut max_y = std::i64::MIN;

//     for &p in subhole.vertices.iter() {
//         min_x = min_x.min(p.x as i64);
//         min_y = min_y.min(p.x as i64);
//         max_x = max_x.max(p.y as i64);
//         max_y = max_y.max(p.y as i64);
//     }

//     let mut set = HashSet::new();

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             let point = Point::new(x as f64, y as f64);

//             print!("{} ", sub_hdc.distance(&point));

//             if sub_hdc.distance(&point) == 0.0 {
//                 set.insert(x * 10000 + y);
//             }
//         }
//         println!();
//     }
//     set
// }

// // #[test]
// fn test_visible_point() {
//     let base = Point::new(1.0, 1.0);

//     let ps = vec![
//         Point::new(0.0, 0.0),
//         Point::new(0.0, 2.0),
//         Point::new(2.0, 2.0),
//         Point::new(2.0, 4.0),
//         Point::new(4.0, 4.0),
//         Point::new(4.0, 0.0),
//     ];
//     let hole = Hole { vertices: ps };

//     let set = enumerate_visible_point(&hole, &base);

//     let mut v = vec![];
//     for i in set.iter() {
//         v.push(*i);
//     }
//     v.sort();

//     for i in v.iter() {
//         println!("{} {}", i / 10000, i % 10000);
//     }

//     assert_eq!(set.len(), 15);
// }
