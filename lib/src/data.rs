#[derive(Clone, Copy)]
pub struct Point {
    y: f64,
    x: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn to_json(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
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
    vertices_: Vec<Point>,
}

impl Hole {
    pub fn new() -> Hole {
        Hole { vertices_: vec![] }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices_.push(p);
    }

    pub fn vertices(&self) -> &Vec<Point> {
        &self.vertices_
    }

    pub fn to_json(&self) -> String {
        vertices_to_json(&self.vertices_)
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
    vertices_: Vec<Point>,
    edges_: Vec<(usize, usize)>,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            vertices_: vec![],
            edges_: vec![],
        }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices_.push(p);
    }

    pub fn connect(&mut self, v1: usize, v2: usize) {
        self.edges_.push((v1, v2));
    }

    pub fn to_json(&self) -> String {
        let mut buffer = String::new();
        buffer += "{";
        buffer += "\"edges\": ";
        buffer += edges_to_json(&self.edges_).as_str();
        buffer += ", \"vertices\": ";
        buffer += vertices_to_json(&self.vertices_).as_str();
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
    vertices_: Vec<Point>,
}

impl Pose {
    pub fn new() -> Hole {
        Hole { vertices_: vec![] }
    }

    pub fn push(&mut self, p: Point) {
        self.vertices_.push(p);
    }

    pub fn vertices(&self) -> &Vec<Point> {
        &self.vertices_
    }

    pub fn to_json(&self) -> String {
        vertices_to_json(&self.vertices_)
    }
}

#[test]
fn test_pose_to_json() {
    let mut pose = Pose::new();
    pose.push(Point::new(2.5, 3.5));
    pose.push(Point::new(4.5, 5.5));
    assert_eq!(pose.to_json(), "[[2.5, 3.5], [4.5, 5.5]]")
}
