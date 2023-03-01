# async-cron-scheduler
Other rust cron schedulers with async support exist but they are getting more feature heavy. This crate is meant to be a simple, lightweight cron scheduler with minimal dependencies to keep the binary size small.

Currently trying to limit the number of dependencies to just `chrono`(https://github.com/chronotope/chrono), `cron` (https://github.com/zslayton/cron), and `tokio` (https://github.com/tokio-rs/tokio).