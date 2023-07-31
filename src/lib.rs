//! Tired of your log lines and progress bars mixing up? indicatif_log_bridge to the rescue!
//!
//! Simply wrap your favourite logging implementation in [crate::LogWrapper]
//!     and those worries are a thing of the past.
//!
//! Just remember to only use progress bars added to the [MultiProgress] you used
//!     , otherwise you are back to ghostly halves of progress bars everywhere.
//!
//! # Example
//! ```rust
//!     # use indicatif_log_bridge::LogWrapper;
//!     # use log::info;
//!     # use indicatif::{MultiProgress, ProgressBar};
//!     # use std::time::Duration;
//!     let logger =
//!         env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
//!             .build();
//!     let multi = MultiProgress::new();
//!
//!     LogWrapper::new(multi.clone(), logger)
//!         .try_init()
//!         .unwrap();
//!
//!     let pg = multi.add(ProgressBar::new(10));
//!     for i in (0..10) {
//!         std::thread::sleep(Duration::from_micros(100));
//!         info!("iteration {}", i);
//!         pg.inc(1);
//!     }
//!     pg.finish();
//!     multi.remove(&pg);
//! ```
//! The code of this crate is pretty simple, so feel free to check it out.

use indicatif::MultiProgress;
use log::Log;

/// Wraps a MultiProgress and a Log implementor
/// calling .suspend on the MultiProgress while writing the log message
/// thereby preventing progress bars and logs from getting mixed up.
///
/// You simply have to add all the progress bars in use to the MultiProgress in use.
pub struct LogWrapper<L: Log> {
    bar: MultiProgress,
    log: L,
}

impl<L: Log + 'static> LogWrapper<L> {
    pub fn new(bar: MultiProgress, log: L) -> Self {
        Self { bar, log }
    }

    /// Installs this as the lobal logger,
    ///
    /// tries to find the correct argument to set_max_level
    /// by reading the logger configuration,
    /// you may want to set it manually though.
    pub fn try_init(self) -> Result<(), log::SetLoggerError> {
        use log::LevelFilter::*;
        let levels = [Off, Error, Warn, Info, Debug, Trace];

        for level_filter in levels.iter().rev() {
            let level = if let Some(level) = level_filter.to_level() {
                level
            } else {
                // off is the last level, just do nothing in that case
                continue;
            };
            let meta = log::Metadata::builder().level(level).build();
            if self.enabled(&meta) {
                log::set_max_level(*level_filter);
                break;
            }
        }

        log::set_boxed_logger(Box::new(self))
    }
    pub fn multi(&self) -> MultiProgress {
        self.bar.clone()
    }
}
impl<L: Log> Log for LogWrapper<L> {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.log.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        self.bar.suspend(|| self.log.log(record))
    }

    fn flush(&self) {
        self.log.flush()
    }
}
