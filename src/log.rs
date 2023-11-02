use tracing::{info, Level};
use tracing_appender::rolling;
use tracing_subscriber::{self, fmt::writer::MakeWriterExt};

pub struct Logger;

impl Logger {
    pub fn init(log_to_file: bool, log_file: Option<String>) {
        if log_to_file {
            let log_file_name = match log_file {
                Some(f) => f,
                None => {
                    panic!("pls give the file name");
                }
            };
            // let debug_file_name =
            let debug_file = rolling::daily("./logs", &log_file_name);
            let warn_file = rolling::daily("logs", &log_file_name).with_max_level(Level::WARN);
            let all_files = debug_file.and(warn_file);
            tracing_subscriber::fmt()
                .with_writer(all_files)
                .with_ansi(false)
                .with_max_level(Level::TRACE)
                .init();
            info!(
                "init Logger component and write content to file: {:?}",
                &log_file_name
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn log_init_test() {
        let to_file = true;
        let log_file = Some("app.log".to_string());
        Logger::init(to_file, log_file);
    }
}
