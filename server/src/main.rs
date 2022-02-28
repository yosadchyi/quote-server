use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use futures::SinkExt;
use rand::seq::IteratorRandom;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

const QUOTES_FILENAME: &str = "data/quotes.txt";

#[derive(Clone)]
struct Config {
    resource: String,
    complexity_bits: u32,
    quotes: Vec<String>,
}

async fn handle_message(config: &Config, socket: TcpStream) {
    let mut lines = Framed::new(socket, LinesCodec::new());

    while let Some(result) = lines.next().await {
        match result {
            Ok(token) => {
                println!("request: {}", token);
                let response = process_token(config, token);
                println!("response: {}", response);
                lines.send(response).await.expect("error sending response");
            }
            Err(_) => {}
        }
    };
    // no more data will be sent, shutdown write connection
    lines.into_inner().shutdown().await.expect("error shutting down connection");
}

fn process_token(config: &Config, token: String) -> &str {
    match hashcash::Token::from_str(token.as_str()) {
        Ok(token) => {
            if token.resource != config.resource {
                "BAD_RESOURCE"
            } else if token.bits < config.complexity_bits {
                "COMPLEXITY_TOO_LOW"
            } else {
                config.quotes.iter().choose(&mut rand::thread_rng()).expect("no quotes")
            }
        }
        Err(_) => "BAD_TOKEN"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let quotes_file = File::open(QUOTES_FILENAME)
        .expect("can't open quotes file");
    let lines = BufReader::new(quotes_file)
        .lines()
        .map(|l| l.expect("error reading line"))
        .collect();

    let config = Config {
        resource: String::from("quote"),
        complexity_bits: 5,
        quotes: lines,
    };

    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let cfg = config.clone();

        tokio::spawn(async move {
            handle_message(&cfg, socket).await;
        });
    }
}
