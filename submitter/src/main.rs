use lib::client::submit_problem;
use lib::data::Pose;
use rayon::prelude::*;

fn main() {
    let max_id = 106;

    let pose_list = (1..=max_id)
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|id| -> Option<Pose> { None })
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
                }
            }
        });
}
