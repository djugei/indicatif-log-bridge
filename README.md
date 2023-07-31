# Indicatif Log Bridge

<!-- cargo-sync-readme start -->

Tired of your log lines and progress bars mixing up? indicatif_log_bridge to the rescue!

Simply wrap your favourite logging implementation in [crate::LogWrapper]
    and those worries are a thing of the past.

Just remember to only use progress bars added to the [MultiProgress] you used
    , otherwise you are back to ghostly halves of progress bars everywhere.

# Example
```rust
    # use indicatif_log_bridge::LogWrapper;
    # use log::info;
    # use indicatif::{MultiProgress, ProgressBar};
    # use std::time::Duration;
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .build();
    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();

    let pg = multi.add(ProgressBar::new(10));
    for i in (0..10) {
        std::thread::sleep(Duration::from_micros(100));
        info!("iteration {}", i);
        pg.inc(1);
    }
    pg.finish();
    multi.remove(&pg);
```
The code of this crate is pretty simple, so feel free to check it out.

<!-- cargo-sync-readme end -->
