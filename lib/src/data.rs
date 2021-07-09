use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone, Copy)]
pub struct Point {
    pub y: f64,
    pub x: f64,
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
}

#[test]
fn test_point_to_json() {
    let p = Point::new(2.5, 3.5);
    let s = p.to_json();
    assert_eq!(s, "[2.5, 3.5]");
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

    pub fn to_json(&self) -> String {
        let mut buffer = String::new();
        buffer += "{";
        buffer += "\"vertices\": ";
        buffer += vertices_to_json(&self.vertices).as_str();
        buffer += "}";
        buffer
    }
}

#[test]
fn test_pose_to_json() {
    let mut pose = Pose::new();
    pose.push(Point::new(2.5, 3.5));
    pose.push(Point::new(4.5, 5.5));
    assert_eq!(pose.to_json(), "{\"vertices\": [[2.5, 3.5], [4.5, 5.5]]}");
}
