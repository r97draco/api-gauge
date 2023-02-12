use reqwest::Error;
use std::time::Instant;
use futures::{stream, StreamExt}; // 0.3.5
use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]
use std::env;

// use serde::Deserialize;
// #[derive(Deserialize, Debug)]
// struct Fact {
//     data: Vec<String>,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut concurrent_requests: usize = 1;
    let mut url: String = "https://google.ca/".to_string();
    if args.len() >= 2 {
        url = args[1].to_string();
        if args.len() == 3 {
            concurrent_requests = args[2].parse().unwrap();
        }
    }
    
    let client = Client::new();
    let urls = vec![url.clone(); concurrent_requests];
    println!("URL is {url}");
    
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
    println!("{}", "-".repeat(50));
    println!("Time elapsed in {concurrent_requests} concurrent HTTP request to URL- {url} is: {:?}", duration);
    println!("Time : {} seconds", duration.as_secs_f64());
    println!("Url : {url}");
    println!("Requests : {concurrent_requests}");
    println!("{}", "-".repeat(50));

    Ok(())
}


// let request_url = format!("https://meowfacts.herokuapp.com/");
// // let request_url = format!("https://r97draco.github.io/Resume");
// println!("{}", request_url);

// let start = Instant::now();
// let response = reqwest::get(&request_url).await?;

// let duration = start.elapsed();

// let facts: Fact = response.json().await?;
// println!("Response JSON: {}", facts.data[0]);

// println!("Time elapsed in apicall is: {:?}", duration);