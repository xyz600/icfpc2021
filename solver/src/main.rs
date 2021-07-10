extern crate lib;

const EPS: f64 = 1e-8;

use lib::algorithm::{next_permutation, HoleDistanceCalculator};
use lib::client::submit_problem;
use lib::data::{Pose, Problem};
use rand::Rng;
use std::time::Instant;

fn is_acceptable(problem: &Problem, vertex_map: &Vec<usize>) -> bool {
    // e というのは、figure の (v1, v2)
    for e in problem.figure.edges.iter() {
        let old_dist = {
            let p0 = problem.figure.vertices[e.0];
            let p1 = problem.figure.vertices[e.1];
            p0.distance2(&p1)
        };
        let new_dist = {
            let p0 = problem.hole.vertices[vertex_map[e.0]];
            let p1 = problem.hole.vertices[vertex_map[e.1]];
            p0.distance2(&p1)
        };
        if (new_dist / old_dist - 1.0).abs() >= problem.epsilon {
            return false;
        }
    }
    true
}

fn solve(problem: &Problem) -> Option<Pose> {
    let n = problem.hole.vertices.len();
    if problem.figure.vertices.len() == n && n <= 12 {
        println!("try solver 1");
        // パターンを全部試して、対応する長さの辺が存在するならその組を出力
        // fiture の座標の i 番目が、hole の vertex_map[i] 番目に相当する
        let mut vertex_map = (0..n).collect::<Vec<usize>>();
        loop {
            if is_acceptable(problem, &vertex_map) {
                let mut pose = Pose::new();
                for i in 0..n {
                    pose.push(problem.hole.vertices[vertex_map[i]]);
                }
                return Some(pose);
            }
            if !next_permutation(&mut vertex_map) {
                break;
            }
        }
        println!("cannot find matching v1.");
        None
    } else {
        println!("vertex size is difference from old one. v1");
        None
    }
}

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
    let mut sum = 100.0 * hdc.distance(&problem.figure.vertices[index]);
    for &ni in problem.figure.neighbors[index].iter() {
        let diff = (problem.figure.distance(ni, index) / pose.distance(ni, index) - 1.0).abs();
        if diff > problem.epsilon * 0.9999 {
            sum += 1e10 * diff;
        }
    }
    sum
}

fn solve2(problem: &Problem, _seed: u64, timeout: u128) -> Option<Pose> {
    // 山登り
    let hdc = HoleDistanceCalculator::new(&problem.hole);

    let timer = Instant::now();
    let mut elapsed_rate =
        (timer.elapsed().as_millis() as f64 + timeout as f64 * 0.5) / (timeout as f64 * 1.5);

    let mut pose = Pose {
        vertices: problem.figure.vertices.clone(),
    };

    let evaluate = |pose: &Pose, index: usize| -> f64 {
        dislike(problem, pose) + penalty(&hdc, problem, pose, index)
    };

    let n = problem.figure.vertices.len();

    let mut rng = rand::thread_rng();
    let mut counter = 0;

    let mut best_eval = std::f64::MAX;

    loop {
        // 乱数で頂点を選択
        let index: usize = rng.gen::<usize>() % n;

        let dist: f64 = rng.gen::<f64>();
        let rad: f64 = rng.gen::<f64>() * std::f64::consts::PI * 2.0;

        let before_eval = evaluate(&pose, index);

        // 頂点座標を移動
        let dx = dist * rad.cos();
        let dy = dist * rad.sin();

        let prev_x = pose.vertices[index].x;
        let prev_y = pose.vertices[index].y;

        pose.vertices[index].x += dx;
        pose.vertices[index].y += dy;

        // コスト差分を計算

        let after_eval = evaluate(&pose, index);

        let eval_diff = after_eval - before_eval;

        // 良ければ山登りで採用
        if eval_diff < 0.0 || (-elapsed_rate * 1e-6 * eval_diff).exp() < rng.gen::<f64>() {
            if after_eval < best_eval {
                println!("{} {} -> {} {}", prev_x, prev_y, prev_x + dx, prev_y + dy);
                println!("improve! eval = {} -> {}", best_eval, after_eval);
                println!(
                    "{} {}",
                    dislike(problem, &pose),
                    penalty(&hdc, problem, &pose, index)
                );

                best_eval = after_eval;
            }
        } else {
            pose.vertices[index].x = prev_x;
            pose.vertices[index].y = prev_y;
        }

        counter += 1;
        if counter % 1024 == 0 {
            if timer.elapsed().as_millis() > timeout {
                break;
            }
            elapsed_rate = (timer.elapsed().as_millis() as f64 + timeout as f64 * 0.5)
                / (timeout as f64 * 1.5);
        }
    }
    eprintln!("counter = {}", counter);
    eprintln!("{}", pose.to_json());

    // 外側に残った点が存在してしまうなら false
    for vertex in pose.vertices.iter() {
        if hdc.distance(&vertex) > 0.0 {
            return None;
        }
    }

    Some(pose)
}

fn main() {
    for id in 1..79 {
        let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());
        println!("load problem {}:", id);
        if let Some(pose) = solve(&problem) {
            println!("submit problem! {}", id);
            if let Err(_msg) = submit_problem(id, &pose) {
                panic!("fail to submit problem {}", id);
            }
        } else if let Some(pose) = solve2(&problem, 0, 30000) {
            println!("submit problem 2! {}", id);
            if let Err(_msg) = submit_problem(id, &pose) {
                panic!("fail to submit problem 2 {}", id);
            }
        } else {
            println!("fail to find solution");
        }
        println!("==========");
    }
}
