use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::sync::Arc;
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
    let task = ScheduledTask::new("* * * * * * *", "test").repeat(5);
    let task = Arc::new(task);
    let mut interval = task.schedule.upcoming(Utc);

    let mut count = 0;

    loop {
        let next = interval.next().unwrap();
        let now = Utc::now();
        let duration = next.signed_duration_since(now);
        sleep(duration.to_std().unwrap()).await;
        println!("task: {:?}", task.handler);

        //loop the number of times of repeat
        count += 1;
        if task.repeat > 0 && count >= task.repeat {
            break;
        }
    }
}
