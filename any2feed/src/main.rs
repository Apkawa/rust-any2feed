use std::{env};


use std::fs::read_to_string;


use any2feed::config::MainConfig;
use any2feed::importers::mewe::importer::MeweImporter;
use any2feed::importers::traits::Importer;


use http_server::{HTTPRequest, HTTPResponse, Route, run, ServerConfig};


fn main_view(_request: &HTTPRequest) -> http_server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content("OK".to_string()))
}

fn main() {
    // todo cli
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        panic!("Need config path arg");
    }
    let config_path = &args[1];
    let config_str = read_to_string(config_path.as_str()).unwrap();
    dbg!(&config_path, &config_str);
    let config: MainConfig = toml::from_str(&config_str).unwrap();

    let mut routes = vec![
        Route::new("/", main_view),
        Route::new("/hello",
                   |_r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];

    let mewe_importers = MeweImporter::with_config(&config_str);
    routes.extend(mewe_importers.routes());

    let run_args = ServerConfig {
        port: config.port,
        routes,
        ..ServerConfig::default()
    };

    run(run_args).unwrap();
}
