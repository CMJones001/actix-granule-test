//! Load configuration from enviroment variables

use config::{self, ConfigError};
use deadpool_postgres::Pool;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use tera::Tera;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;

        cfg.try_into()
    }

    pub fn configure_log(&self) -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(NoTls).unwrap()
    }

    pub fn configure_tera(&self) -> Tera {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing errors: {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    }
}
