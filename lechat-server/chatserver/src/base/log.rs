use std::str::FromStr;

use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;

use crate::{base, Error, Result};

const LOG_ADMIN_TOKEN: &str = "mirco_chat_log";

pub fn init_logger(cfg: &base::config::Config) -> Result<Box<dyn FnOnce()>> {
    let (std_logout, _stdguard) = tracing_appender::non_blocking(std::io::stdout());
    let file_appender = tracing_appender::rolling::daily(&cfg.log.filepath, LOG_ADMIN_TOKEN);
    let (file_logout, _file_guard) = tracing_appender::non_blocking(file_appender);
    let local_time = tracing_subscriber::fmt::time::OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]:[subsecond digits:6]"),
    );
    let file_log_level =
        tracing::Level::from_str(&cfg.log.level).map_err(|e| Error::ConfigError(e.to_string()))?;

    let subscriber = tracing_subscriber::registry()
        .with(
            fmt::Layer::new()
                .with_writer(std_logout.with_max_level(tracing::Level::DEBUG))
                .with_timer(local_time.clone())
                .with_ansi(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .pretty(),
        )
        .with(
            fmt::Layer::new()
                .with_writer(file_logout.with_max_level(file_log_level))
                .with_timer(local_time)
                .with_ansi(false)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .pretty(),
        );
    tracing::subscriber::set_global_default(subscriber).unwrap();

    Ok(Box::new(|| {
        drop(_stdguard);
        drop(_file_guard)
    }))
}
