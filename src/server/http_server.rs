use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use crate::server::config::ServerConfig;
use crate::server::data::{HTTPMethod, HTTPRequest};
use crate::server::HTTPError::*;
use crate::server::response::HTTPResponse;
use crate::server::thread_pool::ThreadPool;


pub(crate) fn handle_client(mut stream: TcpStream, config: Arc<ServerConfig>) {
    // let addr = stream.peer_addr().unwrap();
    let reader = BufReader::new(&stream);
    // TODO check body
    let request: Vec<String> = reader.lines()
        .map(|r| r.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();


    if request.len() == 0 {
        // Close
        return;
    }
    let request = HTTPRequest::parse(&request).unwrap();
    let routes = &config.routes;
    let mut response = Err(NotFound);
    for r in routes {
        if r.match_path(&request.path) {
            response = r.run_cb(&request);
            break;
        }
    }
    dbg!(&response);
    let response = match response {
        Ok(r) => r,
        Err(e) => {
            match e {
                NotFound => HTTPResponse::new(404),
                _ => HTTPResponse::new(500),
            }
        },
    };
    stream.write_all(response.to_string().as_bytes()).unwrap()
}

pub fn run(config: ServerConfig) -> std::io::Result<()> {
    let addr = config.addr();
    println!("Run server: {}", addr);
    let listener = TcpListener::bind(addr)?;

    let pool = ThreadPool::new(4);
    let config = Arc::new(config);
    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _config = Arc::clone(&config);
        pool.execute(move || {
            handle_client(stream, _config);
        })
    }
    Ok(())
}