use chrono::DateTime;
use chrono::Local;

pub fn get_current_datetime() -> DateTime<Local> {
    Local::now()
}