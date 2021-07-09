use crate::data::Pose;

use async_std::task;
use http_client::http_types::{Method, Request};
use http_client::HttpClient;

const API_TOKEN: &str = "09d3f025-0404-45de-8361-b5db9c6bfff6";
const URL: &str = "https://poses.live";

use http_client::h1::H1Client as Client;

pub async fn hello() -> Result<String, String> {
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

#[test]
fn test_hello() {
    let s = task::block_on(hello());
    if let Ok(body) = s {
        assert_eq!(body, "{\"hello\":\"xyz600\"}");
    } else {
        panic!("fail to say hello.");
    }
}

pub async fn get_problem(id: usize) -> Result<String, String> {
    let client = Client::new();

    let mut req = Request::new(
        Method::Get,
        format!("{}/problems/{}/download", URL, id).as_str(),
    );
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

#[test]
fn test_get_problem() {
    let problem = task::block_on(get_problem(1));
    match problem {
        Err(_msg) => panic!("fail to get problem 1"),
        Ok(_problem_json) => {}
    };
}

pub async fn submit_problem(id: usize, pose: &Pose) -> Result<String, String> {
    let client = Client::new();

    let mut req = Request::new(
        Method::Post,
        format!("{}/problems/{}/solutions", URL, id).as_str(),
    );
    req.insert_header("Authorization", format!("Bearer {}", API_TOKEN));
    req.set_body(pose.to_json());
    let maybe_res = client.send(req).await;
    if let Ok(res) = maybe_res {
        let mut res = res;
        let ret = res.body_string().await.unwrap();
        Ok(ret)
    } else {
        Err("fail to get problem".to_string())
    }
}

#[test]
fn test_submit_problem() {
    // cannot test
}
