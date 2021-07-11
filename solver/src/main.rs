extern crate lib;

const EPS: f64 = 1e-8;

use lib::algorithm::{next_permutation, HoleDistanceCalculator};
use lib::client::submit_problem;
use lib::data::{Line, Point, Pose, Problem};
use rand::prelude::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::path::Path;
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
        None
    } else {
        None
    }
}

#[derive(Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Pos {
        Pos { x: x, y: y }
    }

    fn distance(&self, p: &Pos) -> i64 {
        let dy = self.y.max(p.y) - self.y.min(p.y);
        let dx = self.x.max(p.x) - self.x.min(p.x);
        dy * dy + dx * dx
    }
    fn to_point(&self) -> Point {
        Point::new(self.x as f64, self.y as f64)
    }
}

struct SolverProblem {
    hole_distance: Vec<Vec<usize>>,
    height: usize,
    width: usize,

    hole_vertices: Vec<Pos>,
    offset_y: i64,
    offset_x: i64,

    orig_figure_vertices: Vec<Pos>,
    figure_neighbors: Vec<Vec<usize>>,
}

impl SolverProblem {
    fn new(problem: &Problem) -> SolverProblem {
        let mut ret = SolverProblem {
            hole_distance: vec![],
            height: 0,
            width: 0,
            hole_vertices: vec![],
            offset_y: 0,
            offset_x: 0,
            orig_figure_vertices: vec![],
            figure_neighbors: problem.figure.neighbors.clone(),
        };

        // 登場座標が (0, 0) で最小になるような調整
        let mut min_x = std::i64::MAX;
        let mut min_y = std::i64::MAX;

        for p in problem.hole.vertices.iter() {
            min_x = min_x.min(p.x as i64);
            min_y = min_y.min(p.y as i64);
        }
        for p in problem.figure.vertices.iter() {
            min_x = min_x.min(p.x as i64);
            min_y = min_y.min(p.y as i64);
        }

        ret.offset_y = min_y;
        ret.offset_x = min_x;

        for p in problem.hole.vertices.iter() {
            let x = p.x as i64 - min_x;
            let y = p.y as i64 - min_y;

            ret.height = ret.height.max((y + 1) as usize);
            ret.width = ret.width.max((x + 1) as usize);

            ret.hole_vertices.push(Pos::new(x, y));
        }

        for p in problem.figure.vertices.iter() {
            let x = p.x as i64 - min_x;
            let y = p.y as i64 - min_y;

            ret.height = ret.height.max((y + 1) as usize);
            ret.width = ret.width.max((x + 1) as usize);

            ret.orig_figure_vertices.push(Pos::new(x, y));
        }

        let hdc = HoleDistanceCalculator::new(&problem.hole);
        ret.hole_distance.resize(ret.height, vec![0; ret.width]);
        for y in 0..ret.height {
            for x in 0..ret.width {
                let orig_x = (x as i64 + ret.offset_x) as f64;
                let orig_y = (y as i64 + ret.offset_y) as f64;
                let p = Point::new(orig_x, orig_y);
                let d = hdc.distance(&p);
                ret.hole_distance[y][x] = (d * d).round() as usize;
            }
        }
        ret
    }

    fn figure_distance(&self, i: usize, j: usize) -> i64 {
        self.orig_figure_vertices[i].distance(&self.orig_figure_vertices[j])
    }
}

#[derive(Clone)]
struct Solution {
    vertices: Vec<Pos>,
}

impl Solution {
    fn new(init: &Vec<Pos>) -> Solution {
        Solution {
            vertices: init.clone(),
        }
    }

    fn to_pose(&self, problem: &SolverProblem) -> Pose {
        let mut pose = Pose::new();
        for p in self.vertices.iter() {
            let x = (p.x + problem.offset_x) as f64;
            let y = (p.y + problem.offset_y) as f64;
            pose.push(Point::new(x, y));
        }
        pose
    }
}

fn dislike(problem: &SolverProblem, sol: &Solution) -> f64 {
    let mut sum = 0;
    for hv in problem.hole_vertices.iter() {
        let mut dist = std::i64::MAX;
        for pv in sol.vertices.iter() {
            dist = dist.min(pv.distance(hv));
        }
        sum += dist;
    }
    sum as f64
}

fn penalty(problem: &SolverProblem, sol: &Solution, epsilon: f64) -> (f64, f64, f64) {
    // 穴の内部からの距離
    let mut p0 = 0.0;
    for pos in sol.vertices.iter() {
        p0 += problem.hole_distance[pos.y as usize][pos.x as usize] as f64;
    }
    // 頂点間の距離
    let mut p1 = 0.0;
    for i in 0..sol.vertices.len() {
        for &ni in problem.figure_neighbors[i].iter() {
            let orig_dist = problem.figure_distance(i, ni);
            let cur_dist = sol.vertices[i].distance(&sol.vertices[ni]);
            let rate = (cur_dist as f64 / orig_dist as f64 - 1.0).abs();
            if rate > epsilon {
                p1 += rate * orig_dist as f64;
            }
        }
    }
    // 構成する辺が、hole の辺と被ってはいけない
    let mut p2 = 0.0;
    let n = sol.vertices.len();
    let m = problem.hole_vertices.len();

    for i in 0..n {
        for &j in problem.figure_neighbors[i].iter() {
            let v1 = sol.vertices[i].to_point();
            let v2 = sol.vertices[j].to_point();
            let l1 = Line::new(v1, v2);

            for hi in 0..m {
                let nhi = (hi + 1) % m;
                let v3 = problem.hole_vertices[hi].to_point();
                let v4 = problem.hole_vertices[nhi].to_point();
                let l2 = Line::new(v3, v4);

                if l1.intersect(&l2) {
                    p2 += 1.0;
                }
            }
        }
    }

    (p0, p1, p2)
}

fn evaluate_all(problem: &SolverProblem, sol: &Solution, epsilon: f64) -> f64 {
    let (p0, p1, p2) = penalty(problem, sol, epsilon);

    let scale = 1e-4;
    let score_penalty_rate = 100.0;
    let p01_rate = 10.0;
    let p02_rate = 100.0;

    (dislike(problem, sol) + (p0 + p1 * p01_rate + p2 * p02_rate) * score_penalty_rate) * scale
}

fn save_to_best(problem: &SolverProblem, solution: &Solution, problem_id: usize) {
    let best_filepath = format!("data/best/{}.json", problem_id);

    if !Path::new(best_filepath.as_str()).exists() {
        println!("create new file problem {}", problem_id);
        solution
            .to_pose(problem)
            .save_file(best_filepath.to_string());
        return;
    }

    let best_pose = Pose::from_file(best_filepath.as_str());
    let mut best_solution = Solution { vertices: vec![] };
    for v in best_pose.vertices.iter() {
        best_solution.vertices.push(Pos::new(
            v.x as i64 - problem.offset_x,
            v.y as i64 - problem.offset_y,
        ));
    }

    let best_eval = dislike(problem, &best_solution);
    let new_eval = dislike(problem, &solution);
    if best_eval > new_eval {
        println!(
            "update! problem {}: {} -> {}",
            problem_id, best_eval, new_eval
        );
        solution.to_pose(problem).save_file(best_filepath);
    }
}

fn solve2(_problem: &Problem, _seed: u64, timeout: u128, problem_id: usize) -> Option<Pose> {
    let problem = SolverProblem::new(_problem);

    let n = problem.orig_figure_vertices.len();
    let mut rng = rand::thread_rng();

    let mut counter = 0;

    let timer = Instant::now();
    let mut elapsed_rate = 0.0;

    let mut current_solution = Solution::new(&problem.orig_figure_vertices);
    let mut current_eval = evaluate_all(&problem, &current_solution, _problem.epsilon);

    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    let mut best_solution = current_solution.clone();
    let mut best_eval = std::f64::MAX;

    let accept = |de: f64, elapsed_rate: f64, rng: &mut ThreadRng| -> bool {
        if de < 0.0 {
            true
        } else {
            let rate = rng.gen::<f64>();
            rate < (-de * (0.5 + 0.5 * elapsed_rate) / 1.0).exp()
        }
    };

    loop {
        let method = rng.gen::<usize>() % 1000;

        if method <= 950 {
            // 1頂点の場所移動
            // 90%

            // 頂点を選択
            let v = rng.gen::<usize>() % n;

            // 移動方向を選択
            let dir = rng.gen::<usize>() % 4;

            let ny = current_solution.vertices[v].y + dy[dir];
            let nx = current_solution.vertices[v].x + dx[dir];

            if !(0 <= ny && ny < problem.height as i64 && 0 <= nx && nx < problem.width as i64) {
                continue;
            }
            current_solution.vertices[v].y = ny;
            current_solution.vertices[v].x = nx;

            // 移動してコストを計算
            let after_eval = evaluate_all(&problem, &current_solution, _problem.epsilon);
            let de = after_eval - current_eval;

            // コストが改善するなら移動
            if accept(de, elapsed_rate, &mut rng) {
                current_eval = after_eval;

                if best_eval > current_eval {
                    best_eval = current_eval;
                    best_solution = current_solution.clone();
                }
            } else {
                current_solution.vertices[v].y -= dy[dir];
                current_solution.vertices[v].x -= dx[dir];
            }
        } else {
            // 隣接頂点の swap

            // 頂点を選択
            let v = rng.gen::<usize>() % n;
            let nv = rng.gen::<usize>() % problem.figure_neighbors[v].len();

            // 2座標の swap
            current_solution.vertices.swap(v, nv);

            // 移動してコストを計算
            let after_eval = evaluate_all(&problem, &current_solution, _problem.epsilon);
            let de = after_eval - current_eval;

            // コストが改善するなら移動
            if accept(de, elapsed_rate, &mut rng) {
                current_eval = after_eval;

                if best_eval > current_eval {
                    best_eval = current_eval;
                    best_solution = current_solution.clone();
                }
            } else {
                current_solution.vertices.swap(v, nv);
            }
        }

        counter += 1;
        if counter % 1024 == 1023 {
            let elapsed = timer.elapsed().as_millis();
            if elapsed > timeout {
                break;
            }
            elapsed_rate = elapsed as f64 / timeout as f64;
        }

        if counter % 16384 == 0 {
            // 書き戻し
            current_solution = best_solution.clone();
            current_eval = best_eval;
        }
    }

    println!("counter = {}", counter);
    println!("score: {}", dislike(&problem, &best_solution));
    let (p0, p1, p2) = penalty(&problem, &best_solution, _problem.epsilon);
    println!("penalty: {} {} {}", p0, p1, p2);

    let pose = best_solution.to_pose(&problem);
    if p0 + p1 + p2 < EPS {
        save_to_best(&problem, &best_solution, problem_id);
        Some(pose)
    } else {
        pose.save_file(format!("data/out/{}.json", problem_id));
        None
    }
}

fn main() {
    if false {
        let id = 3;
        let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());
        println!("load problem {}:", id);
        if let Some(pose) = solve2(&problem, 0, 10000, id) {
            pose.save_file(format!("data/out/{}.json", id));
        }
        return;
    }

    let max_id = 106;

    {
        // solve
        (1..=max_id)
            .collect::<Vec<usize>>()
            .par_iter()
            .for_each(|id| {
                let problem = Problem::from_file(format!("data/in/{}.json", id).as_str());
                println!("load problem {}:", id);
                if let Some(_pose) = solve(&problem) {
                } else if let Some(_pose) = solve2(&problem, 0, 60000, *id) {
                }
            });
    }
}
