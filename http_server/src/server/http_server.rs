use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::Arc;

use crate::server::config::ServerConfig;

use crate::server::request::HTTPRequest;
use crate::server::response::HTTPResponse;
use crate::server::thread_pool::ThreadPool;
use crate::server::HTTPError::*;

pub(crate) fn handle_client(mut stream: TcpStream, config: Arc<ServerConfig>) {
    // let addr = stream.peer_addr().unwrap();
    let reader = BufReader::new(&stream);
    // TODO check body
    let request: Vec<String> = reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    if request.is_empty() {
        // Close
        return;
    }
    let mut request = HTTPRequest::parse(&request).unwrap();
    let req_headers = request.headers.clone();
    request.config = Some(Arc::clone(&config));
    request.stream = Some(Box::new(&stream));

    let routes = &config.routes;
    let mut response = Err(NotFound);
    for r in routes {
        if let Some(path_params) = r.parse_path(&request.path) {
            request.path_params = Some(path_params);
            response = r.run_cb(&request);
            break;
        }
    }
    // dbg!(&response);
    let response = match response {
        Ok(r) => r,
        Err(e) => match e {
            NotFound => HTTPResponse::new(404),
            _ => HTTPResponse::new(500),
        },
    };
    println!(
        "{code} {path}",
        code = response.status,
        path = request.full_path
    );
    // TODO Улучшить обработку ошибок.
    let header_write_state = stream
        .write_all(response.to_string().as_bytes())
        .map_err(|e| {
            dbg!(&e, &req_headers, &response);
            e
        });
    if header_write_state.is_ok() {
        let empty_bytes = bytes::Bytes::new();
        let _ = stream
            .write_all(response.content.as_ref().unwrap_or(&empty_bytes))
            .map_err(|e| {
                match e.kind() {
                    io::ErrorKind::BrokenPipe => (), // Разрыв соединения от клиента, пока глушим их
                    _ => {
                        dbg!(&e, &req_headers, &response);
                    } // Другие ошибки
                }
                e
            });
    }
}

pub fn run(config: ServerConfig) -> io::Result<()> {
    let addr = config.addr();
    println!("Run server: http://{}", addr);
    let listener = TcpListener::bind(addr)?;

    let pool = ThreadPool::new(config.threads.unwrap_or(4) as usize);
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
