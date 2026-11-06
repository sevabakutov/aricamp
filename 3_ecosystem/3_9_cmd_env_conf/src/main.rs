use clap::{Parser, command};
use config::{Config, Environment, File};
use crate::LogAppLevels::warn;
use {serde::Deserialize};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, default_value = "config.toml", env = "CONF_FILE")]
    conf: String,
}

#[derive(Debug, Deserialize)]
enum LogAppLevels {
    error,
    warn,
    info,
    debug,
    trace
}

#[derive(Debug, Deserialize)]
struct ModeConfig {
    debug: bool,
}

#[derive(Deserialize, Debug)]
struct ServerConfig {
    external_url: String,
    http_port: usize,
    grpc_port: usize,
    healthz_port: usize,
    metrics_port: usize,
}

#[derive(Debug, Deserialize)]
struct MySqlConfig {
    host: String,
    port: usize,
    dating: String,
    user: String,
    pass: String,
    connections: MySqlConnectionsConfig,

}

#[derive(Debug, Deserialize)]
struct MySqlConnectionsConfig {
    max_idle: usize,
    max_open: usize,
}

#[derive(Debug, Deserialize)]
struct DbConfig {
    mysql: MySqlConfig,
}

#[derive(Debug, Deserialize)]
struct LogAppConfig {
    level: LogAppLevels,
}

#[derive(Debug, Deserialize)]
struct LogConfig {
    app: LogAppConfig,
}

#[derive(Debug, Deserialize)]
struct BackgroundWatchdogConfig {
    period: String,
    limit: usize,
    lock_timeout: String,
}

#[derive(Debug, Deserialize)]
struct BackgroundConfig {
    watchdog: BackgroundWatchdogConfig,
}

#[derive(Debug, Deserialize)]
struct Settings {
    mode: ModeConfig,
    server: ServerConfig,
    db: DbConfig,
    log: LogConfig,
    background: BackgroundConfig,
}

fn main() -> Result<(), config::ConfigError> {

    let cli = Cli::parse();

    let settings = Config::builder()
        .set_default("mode.debug", false)?

        .set_default("server.external_url", "http://127.0.0.1")?
        .set_default("server.http_port", 8081)?
        .set_default("server.grpc_port", 8082)?
        .set_default("server.healthz_port", 10025)?
        .set_default("server.metrics_port", 9199)?

        .set_default("db.mysql.host", "127.0.0.1")?
        .set_default("db.mysql.port", 3306)?
        .set_default("db.mysql.dating", "default")?
        .set_default("db.mysql.user", "root")?
        .set_default("db.mysql.pass", "")?
        .set_default("db.mysql.connections.max_idle", 30)?
        .set_default("db.mysql.connections.max_open", 30)?

        .set_default("log.app.level", "info")?

        .set_default("background.watchdog.period", "5s")?
        .set_default("background.watchdog.limit", 10)?
        .set_default("background.watchdog.lock_timeout", "4s")?

        .add_source(File::with_name("config").required(false))

        .add_source(
            Environment::with_prefix("CONF")
                .separator("__")
        )
        .set_override("mode.debug", cli.debug)?
        .build()?;

    let config: Settings = settings.try_deserialize()?;

    println!("{:#?}", config);

    Ok(())
}
