use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::env;
use std::error::Error;
use tokio_util::codec::{Framed, LinesCodec};
use futures::SinkExt;
use tokio_stream::StreamExt;

struct Config {
    resource: String,
    complexity_bits: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let count = env::args()
        .nth(2)
        .unwrap_or_else(|| "1".to_string())
        .parse::<i32>()
        .ok()
        .expect("count expected to be integer number");

    let config = Config {
        resource: String::from("quote"),
        complexity_bits: 5,
    };

    println!("connecting to server...");
    let stream = TcpStream::connect(&addr).await
        .expect("connection error");

    // generate 'count' tokens and send them
    let mut lines = Framed::new(stream, LinesCodec::new());

    for _ in 0..count {
        println!("generating token...");
        let token = hashcash::Token::new(config.resource.clone(), config.complexity_bits);
        println!("token generated: {}", token.to_string());
        lines.send(token.to_string()).await.expect("send error");
    }

    let mut stream = lines.into_inner();
    // no more data will be sent, shutdown write connection
    stream.shutdown().await.expect("error shutting down write connection");

    let mut lines= Framed::new(stream, LinesCodec::new());
    while let Some(result) = lines.next().await {
        match result {
            Ok(line) => {
                println!("response: {}", line);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
