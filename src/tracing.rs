use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt::format::FmtSpan,
    fmt::time,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

use crate::config::{TracingFormat, TracingLevelFilter};

pub fn init_subscriber(config: &crate::config::Tracing) {
    let targets = filter(config);
    let registry = tracing_subscriber::registry();

    let fmt_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::NONE)
        .with_target(true)
        .with_timer(time::UtcTime::rfc_3339());

    let fmt_subscriber = match config.format {
        TracingFormat::Json => fmt_subscriber.json().with_filter(targets).boxed(),
        TracingFormat::Pretty => fmt_subscriber.pretty().with_filter(targets).boxed(),
    };

    let registry = registry.with(fmt_subscriber);
    registry.init();
}

fn filter(config: &crate::config::Tracing) -> Targets {
    let mut targets = Vec::new();

    filter_modules(&mut targets, &config.off, LevelFilter::OFF);
    filter_modules(&mut targets, &config.debug, LevelFilter::DEBUG);
    filter_modules(&mut targets, &config.error, LevelFilter::ERROR);
    filter_modules(&mut targets, &config.warn, LevelFilter::WARN);
    filter_modules(&mut targets, &config.info, LevelFilter::INFO);
    filter_modules(&mut targets, &config.trace, LevelFilter::TRACE);

    let level = match config.level {
        TracingLevelFilter::Off => LevelFilter::OFF,
        TracingLevelFilter::Warn => LevelFilter::WARN,
        TracingLevelFilter::Error => LevelFilter::ERROR,
        TracingLevelFilter::Trace => LevelFilter::TRACE,
        TracingLevelFilter::Debug => LevelFilter::DEBUG,
        TracingLevelFilter::Info => LevelFilter::INFO,
    };

    tracing_subscriber::filter::Targets::new()
        .with_default(level)
        .with_targets(targets)
}

fn filter_modules(
    targets: &mut Vec<(String, tracing_subscriber::filter::LevelFilter)>,
    modules: &Option<Vec<String>>,
    level: tracing_subscriber::filter::LevelFilter,
) {
    if let Some(modules) = modules {
        for module in modules {
            targets.push((module.to_owned(), level));
        }
    }
}
