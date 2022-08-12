use anyhow::Result;
use slog::{o, Discard, Drain, Level, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

pub fn logger(level: Level) -> Result<Logger> {
    let drain = term_drain(level).unwrap_or(discard_drain()?).fuse();
    let logger = Logger::root(drain, o!());
    Ok(logger)
}

fn discard_drain() -> Result<slog_async::Async> {
    let discard = Async::default(Discard);
    Ok(discard)
}

fn term_drain(level: Level) -> Result<slog_async::Async> {
    let decorator = TermDecorator::new().build();
    let term = CompactFormat::new(decorator);
    let drain = Async::default(term.build().filter_level(level).fuse());
    Ok(drain)
}
