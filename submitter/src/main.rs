use lib::client::submit_problem;
use lib::data::Pose;
use rayon::prelude::*;

use std::path::Path;

fn main() {
    let max_id = 106;

    let pose_list = (1..=max_id)
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|id| -> Option<Pose> {
            let best_filepath = format!("data/best/{}.json", id);
            if !Path::new(best_filepath.as_str()).exists() {
                None
            } else {
                Some(Pose::from_file(best_filepath.as_str()))
            }
        })
        .collect::<Vec<Option<Pose>>>();

    // submit
    pose_list
        .par_iter()
        .enumerate()
        .for_each(|(id, maybe_pose)| {
            let id = id + 1;
            if let Some(pose) = maybe_pose {
                if let Err(_msg) = submit_problem(id, pose) {
                    println!("fail to submit problem {}", id);
                } else {
                }
            }
        });
}
