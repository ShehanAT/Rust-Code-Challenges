use chrono::{DateTime, Local, Utc, Date, TimeZone};

struct ImportantEvent {
    // TODO: define data structure 
    what: String,
    when: Date<Local>
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {

    fn is_passed(&self) -> bool {
        let currentDate = Local::today();
        if currentDate > self.when {
            return true;
        } else {
            return false;
        }
    }
    
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.ymd(2020, 12, 25),
    };

    if missed_christmas.is_passed() {
        println!("Oh well, maybe next year");
    } else {
        println!("Nice");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}