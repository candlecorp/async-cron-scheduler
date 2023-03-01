use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use tokio::time::sleep;

#[derive(Debug)]
struct ScheduledTask<'a> {
    schedule: Schedule,
    repeat: i16,
    handler: &'a str,
}

impl<'a> ScheduledTask<'a> {
    fn new(cron_with_seconds: &str, handler: &'a str) -> Self {
        let schedule = Schedule::from_str(cron_with_seconds).unwrap();
        Self {
            schedule,
            repeat: 0,
            handler,
        }
    }

    fn repeat(mut self, repeat: i16) -> Self {
        self.repeat = repeat;
        self
    }
}

#[tokio::main]
async fn main() {
    let task = ScheduledTask::new("*/3 * * * * * *", "test").repeat(5);
    let repeat = task.repeat;
    let task = Arc::new(task);
    let mut interval = task.schedule.upcoming(Utc);

    let mut count = 0;
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    loop {
        let next = interval.next().unwrap();
        let now = Utc::now();
        let duration = next.signed_duration_since(now);

        // Wait until the next scheduled time. Do not spawn a new thread or go to next loop until after this
        // task has been scheduled/thread spawned.  Otherwise it could result in multiple threads being spawned
        // for the same time.
        sleep(duration.to_std().unwrap()).await;

        let task = Arc::clone(&task);
        let handle = thread::spawn(move || {
            println!("task: {:?}", task.handler);
        });

        handles.push(handle);

        //loop the number of times of repeat
        count += 1;
        if repeat > 0 && count >= repeat {
            break;
        }
    }

    // Join all the threads in the vector after the loop has completed.
    for handle in handles {
        handle.join().unwrap();
    }
}
