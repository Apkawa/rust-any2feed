use clap::Parser;
use std::fs::read_to_string;

use any2feed::cli::{Commands, CLI};
use any2feed::config::MainConfig;
use any2feed::importers::ImporterList;

use http_server::{run, HTTPRequest, HTTPResponse, Route, ServerConfig};

fn main_view(_request: &HTTPRequest) -> http_server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content(
        r#"<html>
        <body>
            <h1>Feeds:</h1>
            <ul>
                <li><a href="/mewe.opml">Mewe OPML</a></li>
                <li><a href="/telegram.opml">Telegram OPML</a></li>
            </ul>
        </body>
    </html>
    "#,
    )
    .set_content_type("text/html"))
}

fn main() {
    let cli = CLI::parse();
    let config_str = read_to_string(cli.config).unwrap();
    let mut config: MainConfig = toml::from_str(&config_str).unwrap();
    match cli.command {
        Commands::Run(server_cfg) => {
            config.server.port = server_cfg.port;
            config.server.threads = server_cfg.threads;
        }
    }

    let mut routes = vec![Route::new("/", main_view)];

    let importer_list = ImporterList::get_importers(&config_str);
    for importer in importer_list {
        routes.extend(importer.routes());
    }

    let run_args = ServerConfig {
        port: config.server.port,
        threads: config.server.threads,
        routes,
    };

    run(run_args).unwrap();
}
