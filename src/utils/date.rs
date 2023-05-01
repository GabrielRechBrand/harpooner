use chrono::DateTime;
use chrono::Local;

pub fn get_current_datetime() -> DateTime<Local> {
    Local::now()
}

pub fn main() {
    let current_datetime = get_current_datetime();
    println!("The current date and time is: {}", current_datetime);
}