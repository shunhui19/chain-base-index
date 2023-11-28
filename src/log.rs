use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber;

pub struct Logger;

impl Logger {
    pub fn init(log_to_file: bool, log_file: Option<String>) {
        if log_to_file {
            let log_file_name = match log_file {
                Some(mut f) => {
                    f.push_str(".log");
                    f
                }
                None => {
                    panic!("pls give the file name");
                }
            };
            let log_file = rolling::daily("logs", log_file_name);
            tracing_subscriber::fmt()
                .with_writer(log_file)
                .with_ansi(false)
                .with_max_level(Level::TRACE)
                .init();
            return;
        }
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn log_init_test() {
        use tracing::{debug, error, info, trace, warn};
        let to_file = false;
        let log_file = Some("app".to_string());
        Logger::init(to_file, log_file);
        error!("error log");
        warn!("warning log");
        info!("info log");
        debug!("debug log");
        trace!("trace log");
    }
}
