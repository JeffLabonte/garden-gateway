const DATETIME_FORMAT: &str = "%b %-d, %-I:%M:%s";

pub fn println_now(action: &str, board: &str) {
    let now = chrono::Local::now();
    let now_utc = chrono::Utc::now();
    println!(
        "{} - Running: {} pin to {}",
        now.format(DATETIME_FORMAT),
        action,
        board,
    );
    print!("Time in UTC: {}", now_utc.format(DATETIME_FORMAT));
}
