#[allow(dead_code)]
use reqwest::Error;
use serde::Deserialize;
use std::time::Instant;
use futures::{stream, StreamExt}; // 0.3.5
use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]
use std::env;

// const concurrent_requests: usize = 20;

#[derive(Deserialize, Debug)]
struct Fact {
    data: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut concurrent_requests: usize = 20;
    if args.len() == 2 {
        concurrent_requests = args[1].parse().unwrap();
    } 
    
    let client = Client::new();
    let url_vec= vec![("https://meowfacts.herokuapp.com/"), ("https://google.ca/"),
    ("https://r97draco.github.io/Resume"), ("https://Bing.com/"), ("https://DuckDuckGo.com/")];
    let urls = vec![url_vec[0]; concurrent_requests];
    println!("URL is {}", url_vec[1]);
    // let urls = vec!["https://google.ca/"; concurrent_requests];

    
    let start = Instant::now();
    
    let bodies = stream::iter(urls)
    .map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    })
    .buffer_unordered(concurrent_requests);
    
    bodies
    .for_each(|b| async {
        match b {
            Ok(b) => println!("Got {} bytes", b.len()),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    })
    .await;
    let duration = start.elapsed();
    println!("Time elapsed in {concurrent_requests} concurrent apicalls is: {:?}", duration);

    // let request_url = format!("https://meowfacts.herokuapp.com/");
    // // let request_url = format!("https://r97draco.github.io/Resume");
    // println!("{}", request_url);

    // let start = Instant::now();
    // let response = reqwest::get(&request_url).await?;

    // let duration = start.elapsed();

    // let facts: Fact = response.json().await?;
    // println!("Response JSON: {}", facts.data[0]);

    // println!("Time elapsed in apicall is: {:?}", duration);
    Ok(())
}