extern crate lib;

const EPS: f64 = 1e-8;

use lib::algorithm::HoleDistanceCalculator;
use lib::data::{Point, Pose, Problem};
use std::fs::File;
use std::io::{BufWriter, Write};

fn dislike(problem: &Problem, pose: &Pose) -> f64 {
    let mut sum = 0.0;
    for hv in problem.hole.vertices.iter() {
        let mut dist = std::f64::MAX;
        for pv in pose.vertices.iter() {
            dist = dist.min(pv.distance(hv));
        }
        sum += dist;
    }
    sum
}

fn penalty(hdc: &HoleDistanceCalculator, problem: &Problem, pose: &Pose, index: usize) -> f64 {
    let p = Point::new(pose.vertices[index].x, pose.vertices[index].y);
    let mut sum = 100.0 * hdc.distance(&p);
    // 周囲の辺の距離
    for &ni in problem.figure.neighbors[index].iter() {
        let diff = (problem.figure.distance(ni, index) / pose.distance(ni, index) - 1.0).abs();
        if diff > problem.epsilon * 0.9999 {
            // sum += 1e8 * diff;
        }
    }
    sum
}

fn main() {
    for id in 1..79 {
        let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());

        let hdc = HoleDistanceCalculator::new(&problem.hole);
        let mut max_x = std::f64::MIN;
        let mut max_y = std::f64::MIN;
        let mut min_x = std::f64::MAX;
        let mut min_y = std::f64::MAX;
        for &v in problem.hole.vertices.iter() {
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
            min_x = min_x.min(v.x);
            min_y = min_y.min(v.y);
        }
        for &v in problem.figure.vertices.iter() {
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
            min_x = min_x.min(v.x);
            min_y = min_y.min(v.y);
        }
        let grid_size = 100;
        let target_id = 1;
        let mut pose = Pose {
            vertices: problem.figure.vertices.clone(),
        };

        let mut writer =
            BufWriter::new(File::create(format!("data/debug/penalty_map_{}.txt", id)).unwrap());
        let mut buffer = String::new();
        for iy in 0..=grid_size {
            let y = (max_y * iy as f64 + min_y * (grid_size - iy) as f64) / grid_size as f64;
            for ix in 0..=grid_size {
                let x = (max_x * ix as f64 + min_x * (grid_size - ix) as f64) / grid_size as f64;
                pose.vertices[target_id].x = x;
                pose.vertices[target_id].y = y;
                buffer += format!(" {}", penalty(&hdc, &problem, &pose, target_id)).as_str();
            }
            buffer += "\n";
        }
        if let Err(_msg) = writer.write(buffer.as_bytes()) {
            panic!("fail to save result");
        }
    }
}
