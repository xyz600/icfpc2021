extern crate lib;

const EPS: f64 = 1e-8;

use lib::algorithm::HoleDistanceCalculator;
use lib::data::Problem;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    for id in 1..79 {
        let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());
        let hdc = HoleDistanceCalculator::new(&problem.hole);

        let mut writer =
            BufWriter::new(File::create(format!("data/debug/hole_{}.txt", id)).unwrap());
        let mut buffer = String::new();
        for tri in hdc.decomposed_triangles.iter() {
            buffer += format!(
                "{} {} {} {} {} {}",
                tri.v0.x, tri.v0.y, tri.v1.x, tri.v1.y, tri.v2.x, tri.v2.y
            )
            .as_str();
            buffer += "\n";
        }
        if let Err(_msg) = writer.write(buffer.as_bytes()) {
            panic!("fail to save result");
        }
    }
}
