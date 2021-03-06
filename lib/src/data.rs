use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::ops::{Add, Div, Mul, Sub};

const EPS: f64 = 1e-8;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn to_json(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }

    pub fn distance2(&self, p2: &Point) -> f64 {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, p2: &Point) -> f64 {
        self.distance2(p2).sqrt()
    }

    pub fn dot(&self, p: &Point) -> f64 {
        self.x * p.x + self.y * p.y
    }

    pub fn cross(&self, p: &Point) -> f64 {
        self.x * p.y - self.y * p.x
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Point {
        *self / self.norm()
    }

    pub fn eq(&self, p: &Point) -> bool {
        (self.x - p.x).abs() < EPS && (self.y - p.y).abs() < EPS
    }

    pub fn ccw(a: &Point, b: &Point, c: &Point) -> i64 {
        let ab = *b - *a;
        let ac = *c - *a;
        let cross_bc = ab.cross(&ac);
        if cross_bc > 0.0 {
            return 1;
        }
        if cross_bc < 0.0 {
            return -1;
        }
        if ab.dot(&ac) < 0.0 {
            // c--a--b
            return 2;
        }
        if ab.norm() < ac.norm() {
            // a--b--c
            return -2;
        }
        // a--c--b
        0
    }
}

#[test]
fn test_ccw() {
    let v0 = Point::new(35.0, 5.0);
    let v1 = Point::new(95.0, 95.0);
    let v2 = Point::new(65.0, 95.0);
    let v3 = Point::new(45.0, 80.0);

    let ccw_01 = Point::ccw(&v0, &v1, &v3);
    let ccw_12 = Point::ccw(&v1, &v2, &v3);
    let ccw_20 = Point::ccw(&v2, &v0, &v3);

    assert!(ccw_01 > 0);
    assert!(ccw_12 > 0);
    assert!(ccw_20 < 0);
}

impl Add for Point {
    type Output = Self;
    fn add(self, p: Point) -> Point {
        Point::new(self.x + p.x, self.y + p.y)
    }
}

impl Add<f64> for Point {
    type Output = Self;
    fn add(self, v: f64) -> Point {
        Point::new(self.x + v, self.y + v)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, p: Point) -> Point {
        Point::new(self.x - p.x, self.y - p.y)
    }
}

impl Sub<f64> for Point {
    type Output = Self;
    fn sub(self, v: f64) -> Point {
        Point::new(self.x - v, self.y - v)
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, v: f64) -> Point {
        Point::new(self.x * v, self.y * v)
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, v: f64) -> Point {
        Point::new(self.x / v, self.y / v)
    }
}

#[test]
fn test_point_to_json() {
    let p = Point::new(2.5, 3.5);
    let s = p.to_json();
    assert_eq!(s, "[2.5, 3.5]");
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    p0: Point,
    p1: Point,
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Line {
        Line { p0: p0, p1: p1 }
    }
    pub fn dx(&self) -> f64 {
        self.p1.x - self.p0.x
    }
    pub fn dy(&self) -> f64 {
        self.p1.y - self.p0.y
    }

    pub fn has_same_edge(&self, line: &Line) -> bool {
        self.p0.eq(&line.p0) || self.p0.eq(&line.p1) || self.p1.eq(&line.p0) || self.p1.eq(&line.p1)
    }

    pub fn on_same_line(&self, line: &Line) -> bool {
        let ccw1 = Point::ccw(&self.p0, &self.p1, &line.p0);
        let ccw2 = Point::ccw(&self.p0, &self.p1, &line.p1);

        (ccw1 + 2) % 2 == 0 && (ccw2 + 2) % 2 == 0
    }

    pub fn intersect_without_edge(&self, line: &Line) -> bool {
        let a = self.p0;
        let b = self.p1;
        let c = line.p0;
        let d = line.p1;

        Point::ccw(&a, &b, &c) * Point::ccw(&a, &b, &d) < 0
            && Point::ccw(&c, &d, &a) * Point::ccw(&c, &d, &b) < 0
    }

    pub fn intersect(&self, line: &Line) -> bool {
        // https://www.ioi-jp.org/camp/2017/2017-sp_camp-hide.pdf
        let a = self.p0;
        let b = self.p1;
        let c = line.p0;
        let d = line.p1;

        Point::ccw(&a, &b, &c) * Point::ccw(&a, &b, &d) <= 0
            && Point::ccw(&c, &d, &a) * Point::ccw(&c, &d, &b) <= 0
    }

    pub fn intersect_point(&self, line: &Line) -> Option<Point> {
        if self.intersect(line) {
            let a = self.p0;
            let b = self.p1;
            let c = line.p0;
            let d = line.p1;
            Some(a + (b - a) * (a - c).cross(&(d - c)) / (d - c).cross(&(b - a)))
        } else {
            None
        }
    }

    pub fn cross(&self, line: &Line) -> f64 {
        self.dx() * line.dy() - line.dx() * self.dy()
    }

    pub fn dot(&self, line: &Line) -> f64 {
        self.dx() * line.dx() + self.dy() * line.dy()
    }

    pub fn distance_of(&self, p: &Point) -> f64 {
        let v0 = self.p1 - self.p0;
        let v1 = *p - self.p0;
        let nv0 = v0.normalize();
        let d10 = v1.dot(&nv0);
        if -EPS <= d10 && d10 <= v0.norm() + EPS {
            // ??????????????????????????????????????????
            // ?????????????????????????????????^2 - ??????????????????^2
            let d1 = v1.norm();
            (d1 * d1 - d10 * d10).sqrt()
        } else {
            // ??????????????????????????????
            self.p0.distance(p).min(self.p1.distance(p))
        }
    }
}

#[test]
fn test_line_cross_point() {
    let p0 = Point::new(0.0, 0.0);
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(1.0, 0.0);
    let p3 = Point::new(0.0, 1.0);

    let l0 = Line::new(p0, p1);
    let l1 = Line::new(p2, p3);

    let maybe_c = l0.intersect_point(&l1);

    if let Some(c) = maybe_c {
        assert!((c.x - 0.5).abs() < EPS);
        assert!((c.y - 0.5).abs() < EPS);
    } else {
        panic!();
    }
}

#[test]
fn distance_line_point_distance1() {
    // ???????????????????????????????????????
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(3.0, 3.0);
    let p3 = Point::new(3.0, 1.0);
    let l = Line::new(p1, p2);
    let real_dist = l.distance_of(&p3);
    assert!((real_dist - 2.0f64.sqrt()).abs() < EPS);
}

#[test]
fn distance_line_point_distance2() {
    // ???????????????????????????????????????
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(3.0, 3.0);
    let p3 = Point::new(1.0, 0.0);
    let l = Line::new(p1, p2);
    let real_dist = l.distance_of(&p3);
    assert!((real_dist - 1.0f64).abs() < EPS);
}

#[test]
fn test_line_intersect1() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 1.0);
    let c = Point::new(1.0, 0.0);
    let d = Point::new(0.0, 1.0);
    let ab = Line::new(a, b);
    let cd = Line::new(c, d);
    assert!(ab.intersect(&cd));
}

#[test]
fn test_line_intersect2() {
    // ?????????????????????????????????????????????????????????
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 1.0);
    let c = Point::new(1.0, 0.0);
    let ab = Line::new(a, b);
    let bc = Line::new(b, c);
    assert!(!ab.intersect_without_edge(&bc));
    assert!(ab.intersect(&bc));

    let ac = Line::new(a, c);
    assert!(!ac.intersect_without_edge(&bc));
    assert!(ac.intersect(&bc));
}

#[test]
fn test_line_intersect3() {
    // ?????????????????????????????????????????????????????????
    let a = Point::new(2.0, 0.5);
    let b = Point::new(2.0, 4.0);
    let c = Point::new(0.0, 0.0);
    let d = Point::new(2.0, 2.0);
    let ab = Line::new(a, b);
    let cd = Line::new(c, d);
    assert!(ab.intersect(&cd));
}

#[test]
fn test_line_intersect4() {
    let p0 = Point::new(1.0, 1.0);
    let p1 = Point::new(1.0, 3.0);
    let p2 = Point::new(3.0, 3.0);
    let tri = Triangle::new(p0, p1, p2);
    let g = tri.gravity();
    let l0 = Line::new(p2, p0);
    let l1 = Line::new(g, p1);
    assert!(!l1.intersect(&l0));
    assert!(!l0.intersect(&l1));
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Triangle {
        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
        }
    }

    pub fn gravity(&self) -> Point {
        (self.v0 + self.v1 + self.v2) / 3.0
    }

    pub fn is_internal_of(&self, p: &Point) -> bool {
        let c1 = Point::ccw(&self.v0, &self.v1, p);
        let c2 = Point::ccw(&self.v1, &self.v2, p);
        let c3 = Point::ccw(&self.v2, &self.v0, p);

        c1 * c2 >= 0 && c2 * c3 >= 0
    }

    pub fn distance_of(&self, p: &Point) -> f64 {
        if self.is_internal_of(p) {
            0.0
        } else {
            vec![
                Line::new(self.v0, self.v1),
                Line::new(self.v1, self.v2),
                Line::new(self.v2, self.v0),
            ]
            .iter()
            .map(|l| l.distance_of(p))
            .fold(1e100, f64::min)
        }
    }
}

#[test]
fn test_distance_triangle_point1() {
    // ????????????????????????????????????
    let p0 = Point::new(1.0, 1.0);
    let p1 = Point::new(1.0, 3.0);
    let p2 = Point::new(3.0, 3.0);
    let tri = Triangle::new(p0, p1, p2);
    assert!(tri.is_internal_of(&p0));
    assert!(tri.is_internal_of(&p1));
    assert!(tri.is_internal_of(&p2));
    let p3 = Point::new(3.0, 1.0);
    assert!(!tri.is_internal_of(&p3));
    assert_eq!(tri.distance_of(&p3), 2.0f64.sqrt());
}

#[test]
fn test_distance_triangle_point2() {
    // ????????????????????????????????????
    let p0 = Point::new(1.0, 1.0);
    let p1 = Point::new(1.0, 3.0);
    let p2 = Point::new(3.0, 3.0);
    let tri = Triangle::new(p0, p1, p2);
    assert!(tri.is_internal_of(&p0));
    assert!(tri.is_internal_of(&p1));
    assert!(tri.is_internal_of(&p2));
    let p3 = (p0 + p1 + p2 * 2.0) / 4.0;
    assert!(tri.is_internal_of(&p3));
    assert_eq!(tri.distance_of(&p3), 0.0);
}

#[test]
fn test_distance_triangle_potin3() {
    let p0 = Point::new(2.0, 2.0);
    let p1 = Point::new(4.0, 0.0);
    let p2 = Point::new(0.0, 0.0);
    let tri = Triangle::new(p0, p1, p2);

    let p3 = Point::new(2.0, 4.0);
    assert!(!tri.is_internal_of(&p3));
    assert_eq!(tri.distance_of(&p3), 2.0f64);
}

fn vertices_to_json(vertices: &Vec<Point>) -> String {
    let mut buffer = String::new();
    buffer += "[";
    for (i, p) in vertices.iter().enumerate() {
        if i > 0 {
            buffer += ", ";
        }
        buffer += p.to_json().as_str();
    }
    buffer += "]";
    buffer
}

fn edges_to_json(vertices: &Vec<(usize, usize)>) -> String {
    let mut buffer = String::new();
    buffer += "[";
    for (i, p) in vertices.iter().enumerate() {
        if i > 0 {
            buffer += ", ";
        }
        buffer += format!("[{}, {}]", p.0, p.1).as_str();
    }
    buffer += "]";
    buffer
}

pub struct Hole {
    pub vertices: Vec<Point>,
}

impl Hole {
    pub fn new() -> Hole {
        Hole { vertices: vec![] }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices.push(p);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn to_json(&self) -> String {
        vertices_to_json(&self.vertices)
    }
}

#[test]
fn test_hole_to_json() {
    let mut hole = Hole::new();
    hole.push(Point::new(2.5, 3.5));
    hole.push(Point::new(4.5, 5.5));
    assert_eq!(hole.to_json(), "[[2.5, 3.5], [4.5, 5.5]]")
}

pub struct Figure {
    pub vertices: Vec<Point>,
    pub edges: Vec<(usize, usize)>,
    pub neighbors: Vec<Vec<usize>>,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            vertices: vec![],
            edges: vec![],
            neighbors: vec![],
        }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices.push(p);
    }

    pub fn connect(&mut self, v1: usize, v2: usize) {
        if self.neighbors.len() < v1.max(v2) + 1 {
            self.neighbors.resize(v1.max(v2) + 1, vec![]);
        }
        self.edges.push((v1, v2));
        self.neighbors[v1].push(v2);
        self.neighbors[v2].push(v1);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
    }

    pub fn distance(&self, index1: usize, index2: usize) -> f64 {
        return self.vertices[index1].distance(&self.vertices[index2]);
    }

    pub fn to_json(&self) -> String {
        let mut buffer = String::new();
        buffer += "{";
        buffer += "\"edges\": ";
        buffer += edges_to_json(&self.edges).as_str();
        buffer += ", \"vertices\": ";
        buffer += vertices_to_json(&self.vertices).as_str();
        buffer += "}";
        buffer
    }
}

#[test]
fn figure_test_to_json() {
    let mut figure = Figure::new();
    figure.push(Point::new(2.5, 3.5));
    figure.push(Point::new(4.5, 5.5));
    figure.connect(0, 1);
    assert_eq!(
        "{\"edges\": [[0, 1]], \"vertices\": [[2.5, 3.5], [4.5, 5.5]]}",
        figure.to_json()
    );
}

pub struct Problem {
    pub hole: Hole,
    pub figure: Figure,
    pub epsilon: f64,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            hole: Hole::new(),
            figure: Figure::new(),
            epsilon: 0.0,
        }
    }
    pub fn to_json(&self) -> String {
        let mut buffer = String::new();
        buffer += "{";
        buffer += format!("\"hole\": {}", self.hole.to_json()).as_str();
        buffer += format!(", \"figure\": {}", self.figure.to_json()).as_str();
        buffer += format!(", \"epsilon\": {}", self.epsilon).as_str();
        buffer += "}";
        buffer
    }

    pub fn clear(&mut self) {
        self.hole.clear();
        self.figure.clear();
    }

    pub fn from_file(filepath: &str) -> Problem {
        let file = File::open(filepath).unwrap();
        let mut buf = BufReader::new(file);
        let mut s = String::new();
        match buf.read_to_string(&mut s) {
            Err(_) => panic!("fail to read file {}", filepath),
            Ok(_) => Problem::from_json(s.as_str()),
        }
    }

    pub fn from_json(json: &str) -> Problem {
        let mut problem = Problem::new();
        let v = serde_json::from_str::<Value>(json).unwrap();
        for point in v["hole"].as_array().unwrap() {
            let p = point.as_array().unwrap();
            let x = p[0].as_f64().unwrap();
            let y = p[1].as_f64().unwrap();
            problem.hole.push(Point::new(x, y));
        }
        for point in v["figure"]["edges"].as_array().unwrap() {
            let p = point.as_array().unwrap();
            let v1 = p[0].as_u64().unwrap() as usize;
            let v2 = p[1].as_u64().unwrap() as usize;
            problem.figure.connect(v1, v2);
        }
        for point in v["figure"]["vertices"].as_array().unwrap() {
            let p = point.as_array().unwrap();
            let x = p[0].as_f64().unwrap();
            let y = p[1].as_f64().unwrap();
            problem.figure.vertices.push(Point::new(x, y));
        }
        problem.epsilon = v["epsilon"].as_f64().unwrap() / 1e6;
        problem
    }
}

#[test]
fn test_problem_from_json() {
    let json = "{
        \"hole\": [
        [55, 80], [65, 95], [95, 95], [35, 5], [5, 5],
        [35, 50], [5, 95], [35, 95], [45, 80]
        ],
        \"figure\": {
        \"edges\": [
        [2, 5], [5, 4], [4, 1], [1, 0], [0, 8], [8, 3], [3, 7],
        [7, 11], [11, 13], [13, 12], [12, 18], [18, 19], [19, 14],
        [14, 15], [15, 17], [17, 16], [16, 10], [10, 6], [6, 2],
        [8, 12], [7, 9], [9, 3], [8, 9], [9, 12], [13, 9], [9, 11],
        [4, 8], [12, 14], [5, 10], [10, 15]
        ],
        \"vertices\": [
        [20, 30], [20, 40], [30, 95], [40, 15], [40, 35], [40, 65],
        [40, 95], [45, 5], [45, 25], [50, 15], [50, 70], [55, 5],
        [55, 25], [60, 15], [60, 35], [60, 65], [60, 95], [70, 95],
        [80, 30], [80, 40]
        ]
        },
        \"epsilon\": 150000
        }";
    let problem = Problem::from_json(json);
    assert_eq!(problem.hole.vertices.len(), 9);
    assert_eq!(problem.hole.vertices[0].x, 55.0);
    assert_eq!(problem.figure.vertices.len(), 20);
    assert_eq!(problem.figure.vertices[0].x, 20.0);
    assert_eq!(problem.figure.edges.len(), 30);
    assert_eq!(problem.figure.edges[0].0, 2);
    assert_eq!(problem.figure.neighbors[0].len(), 2);
    assert_eq!(problem.epsilon, 0.15);
}

#[test]
fn test_problem_from_file() {
    let problem = Problem::from_file("../data/in/1.json");
    assert_eq!(problem.hole.vertices.len(), 9);
    assert_eq!(problem.hole.vertices[0].x, 45.0);
}

#[test]
fn test_problem_to_json() {
    let mut problem = Problem::new();
    problem.figure.push(Point::new(2.5, 3.5));
    problem.figure.push(Point::new(4.5, 5.5));
    problem.figure.connect(0, 1);
    problem.hole.push(Point::new(2.5, 3.5));
    problem.hole.push(Point::new(4.5, 5.5));
    assert_eq!("{\"hole\": [[2.5, 3.5], [4.5, 5.5]], \"figure\": {\"edges\": [[0, 1]], \"vertices\": [[2.5, 3.5], [4.5, 5.5]]}, \"epsilon\": 0}", problem.to_json());
}

pub struct Pose {
    pub vertices: Vec<Point>,
}

impl Pose {
    pub fn new() -> Pose {
        Pose { vertices: vec![] }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices.push(p);
    }

    pub fn distance(&self, index1: usize, index2: usize) -> f64 {
        self.vertices[index1].distance(&self.vertices[index2])
    }

    pub fn to_json(&self) -> String {
        let mut buffer = String::new();
        buffer += "{";
        buffer += "\"vertices\": ";
        buffer += vertices_to_json(&self.vertices).as_str();
        buffer += "}";
        buffer
    }

    pub fn from_file(filepath: &str) -> Pose {
        let file = File::open(filepath).unwrap();
        let mut buf = BufReader::new(file);
        let mut s = String::new();
        match buf.read_to_string(&mut s) {
            Err(_) => panic!("fail to read file {}", filepath),
            Ok(_) => Pose::from_json(s.as_str()),
        }
    }

    pub fn from_json(json: &str) -> Pose {
        let mut pose = Pose::new();
        let v = serde_json::from_str::<Value>(json).unwrap();
        for point in v["vertices"].as_array().unwrap() {
            let p = point.as_array().unwrap();
            let x = p[0].as_f64().unwrap();
            let y = p[1].as_f64().unwrap();
            pose.vertices.push(Point::new(x, y));
        }
        pose
    }

    pub fn save_file(&self, filepath: String) {
        let mut writer = BufWriter::new(File::create(filepath.as_str()).unwrap());
        if let Err(_msg) = writer.write(self.to_json().as_bytes()) {
            panic!("fail to save result");
        }
    }
}

#[test]
fn test_pose_to_json() {
    let mut pose = Pose::new();
    pose.push(Point::new(2.5, 3.5));
    pose.push(Point::new(4.5, 5.5));
    assert_eq!(pose.to_json(), "{\"vertices\": [[2.5, 3.5], [4.5, 5.5]]}");
}

#[test]
fn test_pose_from_file() {
    let filepath = "../data/best/11.json";
    let pose = Pose::from_file(filepath);

    assert_eq!(pose.vertices.len(), 3);
}
