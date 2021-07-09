extern crate lib;

use lib::algorithm::next_permutation;
use lib::client::submit_problem;
use lib::data::{Pose, Problem};

fn solve(problem: &Problem) -> Option<Pose> {
    let n = problem.hole.vertices.len();
    if problem.figure.vertices.len() == n && n <= 12 {
        // パターンを全部試して、対応する長さの辺が存在するならその組を出力
        // fiture の座標の i 番目が、hole の vertex_map[i] 番目に相当する
        let mut vertex_map = (0..n).collect::<Vec<usize>>();
        loop {
            let mut success = true;
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
                    success = false;
                }
            }
            if success {
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
        println!("cannot find matching.");
        None
    } else {
        println!("vertex size is difference from old one.");
        None
    }
}

fn main() {
    for id in 1..60 {
        let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());
        if let Some(pose) = solve(&problem) {
            println!("submit problem! {}", id);
            if let Err(_msg) = submit_problem(id, &pose) {
                panic!("fail to submit problem {}", id);
            }
        }
    }
}
