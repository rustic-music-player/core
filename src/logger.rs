use slog;
use slog::Drain;
use slog_term;
use slog_async;

lazy_static! {
    pub static ref logger: slog::Logger = build_logger();
}

fn build_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}