extern crate lib;

use lib::client::get_problem;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    (1..107).collect::<Vec<usize>>().par_iter().for_each(|&id| {
        let maybe_problem = get_problem(id);
        if let Ok(problem) = maybe_problem {
            let file = File::create(format!("data/in/{}.json", id)).unwrap();
            let mut buf = BufWriter::new(file);
            if let Err(_) = buf.write_all(problem.as_bytes()) {
                panic!("fail to write file {}", id);
            }
            println!("finish {}", id);
        } else {
            panic!("problem cannot downloaded.");
        }
    });
}
