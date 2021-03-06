use crate::data::Pose;

use async_std::task;
use http_client::http_types::{Method, Request};
use http_client::HttpClient;

const API_TOKEN: &str = "";
const URL: &str = "https://poses.live";

use http_client::h1::H1Client as Client;

pub async fn async_hello() -> Result<String, String> {
    let client = Client::new();

    let mut req = Request::new(Method::Get, format!("{}/api/hello", URL).as_str());
    req.insert_header("Authorization", format!("Bearer {}", API_TOKEN));
    let maybe_res = client.send(req).await;
    if let Ok(res) = maybe_res {
        let mut res = res;
        let ret = res.body_string().await.unwrap();
        Ok(ret)
    } else {
        Err("fail to say hello".to_string())
    }
}

pub fn hello() -> Result<String, String> {
    task::block_on(async_hello())
}

#[test]
fn test_hello() {
    let s = hello();
    if let Ok(body) = s {
        assert_eq!(body, "{\"hello\":\"xyz600\"}");
    } else {
        panic!("fail to say hello.");
    }
}

pub async fn async_get_problem(id: usize) -> Result<String, String> {
    let client = Client::new();

    let mut req = Request::new(Method::Get, format!("{}/api/problems/{}", URL, id).as_str());
    req.insert_header("Authorization", format!("Bearer {}", API_TOKEN));
    let maybe_res = client.send(req).await;
    if let Ok(res) = maybe_res {
        let mut res = res;
        let ret = res.body_string().await.unwrap();
        Ok(ret)
    } else {
        Err("fail to get problem".to_string())
    }
}

pub fn get_problem(id: usize) -> Result<String, String> {
    task::block_on(async_get_problem(id))
}

#[test]
fn test_get_problem() {
    let problem = task::block_on(async_get_problem(1));
    match problem {
        Err(_msg) => panic!("fail to get problem 1"),
        Ok(_problem_json) => {}
    };
}

pub async fn async_submit_problem(id: usize, pose: &Pose) -> Result<String, String> {
    let client = Client::new();

    let mut req = Request::new(
        Method::Post,
        format!("{}/api/problems/{}/solutions", URL, id).as_str(),
    );
    req.insert_header("Authorization", format!("Bearer {}", API_TOKEN));
    req.set_body(pose.to_json());
    println!("{}", pose.to_json());
    let maybe_res = client.send(req).await;
    if let Ok(res) = maybe_res {
        let mut res = res;
        let ret = res.body_string().await.unwrap();
        Ok(ret)
    } else {
        Err("fail to get problem".to_string())
    }
}

pub fn submit_problem(id: usize, pose: &Pose) -> Result<String, String> {
    task::block_on(async_submit_problem(id, pose))
}

#[test]
fn test_submit_problem() {
    use crate::data::Point;

    const ID: usize = 13;
    let mut pose = Pose::new();
    pose.push(Point::new(20.0, 0.0));
    pose.push(Point::new(40.0, 20.0));
    pose.push(Point::new(0.0, 20.0));
    pose.push(Point::new(20.0, 40.0));

    match submit_problem(ID, &pose) {
        Err(_msg) => panic!("failed to send msg"),
        Ok(body) => println!("{}", body),
    }
}
