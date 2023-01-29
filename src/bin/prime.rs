use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[derive(Deserialize, Debug)]
enum Method {
    #[serde(rename = "isPrime")]
    IsPrime,
}

#[derive(Deserialize, Debug)]
struct Request {
    method: Method,
    number: u32,
}

#[derive(Serialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::var("ADDR").unwrap_or("0.0.0.0:3000".to_string());

    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}.", &addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let mut stream = BufReader::new(stream);

        tokio::spawn(async move {
            loop {
                let mut line = String::new();

                stream
                    .read_line(&mut line)
                    .await
                    .expect("failed to read data from socket");

                match serde_json::from_str::<Request>(&line) {
                    Ok(req) => {
                        let response = Response {
                            method: "isPrime".to_string(),
                            prime: is_prime(req.number),
                        };

                        let serialized = serde_json::to_string(&response).unwrap();

                        stream
                            .write_all(format!("{}\n", serialized).as_bytes())
                            .await
                            .expect("failed to write data to socket");
                    }
                    Err(_) => {
                        let response = ErrorResponse {
                            message: "Malformed request.".to_string(),
                        };

                        let serialized = serde_json::to_string(&response).unwrap();

                        stream
                            .write_all(format!("{}\n", serialized).as_bytes())
                            .await
                            .expect("failed to write data to socket");
                    }
                }
            }
        });
    }
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
