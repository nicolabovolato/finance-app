use std::str::FromStr;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::Layer, prelude::*, EnvFilter, Registry};

// WorkerGuard is used to write any log if the programs closes abruptly
pub fn init(level: &str, pretty: bool) -> WorkerGuard {
    let subscriber = Registry::default();

    let (non_blocking_io, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let json_log = match pretty {
        false => Some(Layer::default().with_writer(non_blocking_io).json()),
        _ => None,
    };

    let pretty_log = match pretty {
        true => Some(Layer::default().pretty()),
        _ => None,
    };

    let subscriber = subscriber
        .with(json_log)
        .with(pretty_log)
        .with(EnvFilter::from_str(level).unwrap());

    subscriber.init();

    _guard
}
