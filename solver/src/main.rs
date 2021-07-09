extern crate lib;

use lib::client::submit_problem;
use lib::data::{Pose, Problem};

fn solve(problem: &Problem) -> Option<Pose> {
    None
}

fn main() {
    for id in 1..60 {
        let problem = Problem::from_file(format!("data/in/{}.json", id));
        if let Some(pose) = solve(&problem) {
            println!("submit problem! {}", id);
            if let Err(msg) = submit_problem(id, &pose) {
                panic!("fail to submit problem {}", id);
            }
        }
    }
}
