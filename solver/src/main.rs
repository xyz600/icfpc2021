extern crate lib;

use lib::algorithm::next_permutation;
use lib::client::submit_problem;
use lib::data::{Pose, Problem};

fn solve(problem: &Problem) -> Option<Pose> {
    if problem.figure.vertices.len() == problem.hole.vertices.len() {
        // パターンを全部試して、対応する長さの辺が存在するならその組を出力
        let n = problem.hole.vertices.len();
        let mut vertex_map = (0..n).collect::<Vec<usize>>();
        loop {
            if !next_permutation(&mut vertex_map) {
                break;
            }
        }
        None
    } else {
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
