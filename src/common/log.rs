use std::fmt;

use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{self, fmt::format::Writer};

pub struct Logger;

impl Logger {
    pub fn init(log_file: Option<String>) {
        match log_file {
            Some(mut f) => {
                f.push_str(".log");
                let log_file = rolling::daily("logs", f);
                let subscriber = tracing_subscriber::fmt()
                    .with_writer(log_file)
                    .with_timer(CustomTimer)
                    .with_ansi(false)
                    .with_max_level(Level::TRACE)
                    .finish();
                _ = tracing::subscriber::set_global_default(subscriber);
            }
            None => {
                let subscriber = tracing_subscriber::fmt()
                    .with_timer(CustomTimer)
                    .with_max_level(Level::TRACE)
                    .finish();
                _ = tracing::subscriber::set_global_default(subscriber);
            }
        }
    }
}

/// Custom time.
struct CustomTimer;

impl tracing_subscriber::fmt::time::FormatTime for CustomTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let now = chrono::Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn log_test() {
        use tracing::{debug, error, info, trace, warn};
        Logger::init(None);
        // Logger::init(Some("app".to_string()));
        error!("error log");
        warn!("warning log");
        sleep(Duration::from_millis(200));
        info!("info log");
        debug!("debug log");
        trace!("trace log");
    }
}
