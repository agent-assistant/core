use chrono;
use chrono::Timelike;
use json;

pub fn time() -> json::JsonValue {
    let time = chrono::Local::now();
    return json::object!{
        "type": "time",
        "icon": "clock",
        "text": format!("It is {}:{} {}", time.hour12().1, time.minute(), if time.hour12().0 == false {"AM"} else {"PM"})
    }
}