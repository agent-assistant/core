// TODO: something todo here...

use json;
use json::object;
use chrono;
use chrono::Timelike;

pub fn parse(input: &str) -> String {
    //! Returns a JSON str of suggestions
    let mut datasrc: Vec<json::JsonValue> = Vec::new();
    match input {
        "what time is it" => {
            let time = chrono::Local::now();
            datasrc.insert(datasrc.len(), object!{
                "type": "time",
                "icon": "clock",
                "text": format!("It is {}:{} {}", time.hour12().1, time.minute(), if time.hour12().0 == false {"AM"} else {"PM"})
            })
        }
        _ => {
            todo!()
        }
    };
    let rtn = json::stringify(datasrc);
    return rtn;
}