# Indicatif Log Bridge

<!-- cargo-rdme start -->

Tired of your log lines and progress bars mixing up? indicatif_log_bridge to the rescue!

Simply wrap your favourite logging implementation in [LogWrapper]
    and those worries are a thing of the past.

Just remember add each [ProgressBar](indicatif::ProgressBar) to the [MultiProgress] you used
    , otherwise you are back to ghostly halves of progress bars everywhere.

## Example
```rust
use env_logger;
use indicatif::{MultiProgress, ProgressBar};
use indicatif_log_bridge::LogWrapper;
use log::info;
use std::time::Duration;

fn main() {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .build();
    let level = logger.filter();
    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();
    log::set_max_level(level);

    let pg = multi.add(ProgressBar::new(10));
    for i in (0..10) {
        std::thread::sleep(Duration::from_secs(1));
        info!("iteration {}", i);
        pg.inc(1);
    }
    pg.finish();
    multi.remove(&pg);
}
```
The code of this crate is pretty simple, so feel free to check it out.


## Known Issues
### Wrong Global Log Level
The log framework has a global minimum level, set using [log::set_max_level].
If that is set to Debug, the trace! macro will not fire at all.
The [Log] trait does not provide a standartized way of querying the expected level.
[LogWrapper::try_init] tries hard to find the correct level, but does not always get it right,
    especially if different levels are specified for different modules or crates,
        as is often the case with the `env_logger` crate.

#### Workaround
For `env_logger` specifically you can use `logger.filter()` to query the level
before constructing and initializing the [LogWrapper] and then passit to [log::set_max_level]
afterwards.
If you copy the [example code](#example) you should be fine.

<!-- cargo-rdme end -->
