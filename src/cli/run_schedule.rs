use crate::helpers::populate_job_ids;
use std::time::Duration;
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
pub async fn run() -> bool {
    let mut scheduler = JobScheduler::new().unwrap();

    let job_ids = populate_job_ids(&scheduler);
    let mut run_again: bool = false;
    loop {
        if job_ids.is_empty() {
            println!("No Jobs to run! Bye!");
            break;
        }

        match scheduler.tick() {
            Ok(_) => match scheduler.time_till_next_job() {
                Ok(v) => match v {
                    Some(_) => {
                        std::thread::sleep(Duration::from_millis(500));
                    }
                    None => {
                        run_again = true;
                        break;
                    }
                },
                Err(e) => {
                    println!("Couldn't retrieve the time till next job: {}", e);
                    run_again = true;
                    break;
                }
            },
            Err(e) => {
                println!("Something went wrong during runtime: {}", e);
                run_again = true;
                break;
            }
        };
    }

    run_again
}
