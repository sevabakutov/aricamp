use std::fs::File;
use tracing::{error, info, warn, span};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    Layer,
};

#[tracing::instrument(target="access")]
fn access_logs(){
    info!(target: "access" ,msg = "123");
}

fn main() {
    let stdout_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::io::stdout)
        .with_filter(tracing_subscriber::filter::filter_fn(|meta| {
            match meta.level() {
                &tracing::Level::ERROR | &tracing::Level::WARN => false,
                _ => meta.target() != "access",
            }
        }));
    let stderr_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::io::stderr)
        .with_filter(EnvFilter::new("warn")
            .add_directive("access=off".parse().unwrap())
            );

    let access_file = File::create("access.log").unwrap();
    let access_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(access_file)
        .with_filter(
            tracing_subscriber::filter::Targets::new()
                .with_target("access", tracing::Level::INFO)
        );

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(stderr_layer)
        .with(access_layer)
        .init();

    {
        let calculation_span = span!(tracing::Level::INFO, "calculations");
        let guard = calculation_span.enter();

        add_two(56);
        add_two(56);
        add_two(56);
        add_two(56);
        add_two(56);
        add_two(56);
        add_two(56);
        add_two(56);
    }
    access_logs();
    access_logs();

    warn!("awdadwdwa");
    error!("awdwadawdawd");
}

fn add_two(a: i64) -> i64 {
    let single_calculation_span = span!(tracing::Level::INFO, "add 2 calculation");
    let guard = single_calculation_span.enter();
    info!("add 2 calculation started");
    a + 2
}
